use gabriele::{machine::Machine, printing::Instruction};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;

pub struct TestApp {
    pub latch: Arc<Mutex<Vec<Instruction>>>,
    #[allow(dead_code)]
    pub machine: Machine,
}

impl TestApp {
    fn new(tx: UnboundedSender<Instruction>) -> Self {
        let machine = Machine::new(tx);
        let latch = Default::default();
        TestApp { machine, latch }
    }
}

struct TestRunner {
    pub receiver: UnboundedReceiver<Instruction>,
}

impl TestRunner {
    pub fn run(&mut self, latch: Arc<Mutex<Vec<Instruction>>>) {
        loop {
            match self.receiver.try_recv() {
                Ok(Instruction::Shutdown) => break,
                Ok(instruction) => {
                    let mut items = latch.lock().unwrap();
                    items.push(instruction);
                }
                _ => {
                    sleep(Duration::from_millis(2));
                    continue;
                }
            }
        }
    }
}

fn start_runner(rx: UnboundedReceiver<Instruction>, latch: Arc<Mutex<Vec<Instruction>>>) {
    let mut runner = TestRunner { receiver: rx };
    runner.run(latch)
}

pub fn start_test_app() -> (TestApp, JoinHandle<()>) {
    let (tx, rx) = mpsc::unbounded_channel::<Instruction>();

    let app = TestApp::new(tx);
    let latch = app.latch.clone();

    let runner_handle = tokio::task::spawn_blocking(move || start_runner(rx, latch));
    (app, runner_handle)
}
