use gabriele::database::Db;
use gabriele::machine::Machine;
use gabriele::printing::Instruction;
use std::fs;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::{spawn, JoinHandle};

pub struct TestApp {
    pub latch: Arc<Mutex<Vec<Instruction>>>,
    pub machine: Machine,
}

impl TestApp {
    fn new(tx: UnboundedSender<Instruction>) -> Self {
        let machine = Machine::new(tx);
        let latch = Default::default();
        TestApp { machine, latch }
    }
}

pub struct TestRunner {
    pub receiver: UnboundedReceiver<Instruction>,
}

impl TestRunner {
    pub async fn run(&mut self, latch: Arc<Mutex<Vec<Instruction>>>) {
        println!("------RUNNER HAS STARTED----");
        loop {
            match self.receiver.recv().await {
                Some(Instruction::Shutdown) => break,
                Some(instruction) => {
                    println!("RECEIVED INSTRUCTION: {:?}", instruction);
                    let mut items = latch.lock().unwrap();
                    items.push(instruction);
                }
                _ => continue,
            }
        }
    }
}

async fn start_runner(rx: UnboundedReceiver<Instruction>, latch: Arc<Mutex<Vec<Instruction>>>) {
    let mut runner = TestRunner { receiver: rx };
    runner.run(latch).await
}

async fn start_test_app() -> (TestApp, JoinHandle<()>) {
    let (tx, rx) = mpsc::unbounded_channel::<Instruction>();

    let app = TestApp::new(tx);
    let latch = app.latch.clone();

    let handle = spawn(async move { start_runner(rx, latch).await });
    (app, handle)
}

fn load_test_db() -> Db {
    let wheel = fs::read_to_string("wheels/German.toml").unwrap();
    toml::from_str(&wheel).unwrap()
}

#[tokio::test]
async fn starts_test_application() {
    let (mut app, runner_handle) = start_test_app().await;
    let db = load_test_db();

    app.machine.print("AT", &db);
    app.machine.shutdown();

    _ = tokio::join!(runner_handle);
    let latch = app.latch.lock().unwrap();

    assert_eq!(latch.len(), 2);
    assert_eq!(latch.get(0).unwrap(), &Instruction::bytes(36, 159));
    assert_eq!(latch.get(1).unwrap(), &Instruction::bytes(37, 159));
}
