use env_logger::{Builder, Target};
use gabriele::database::Db;
use gabriele::hal::Hal;
use gabriele::machine::Machine;
use gabriele::printing::Instruction;
use log::{debug, info};
use std::env;
use std::fs;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

fn welcome(machine: &mut Machine, db: &Db) {
    debug!("Printing text");
    machine.print(
        "\"Il Signore Gesù, Verbo Incarnato,\n\
        ci doni la grazia della gioia nel servizio umile e generoso.\n\
        E per favore, mi raccomando,\n\
        non perdiamo il senso dell’umorismo, che è salute!\"\n\
        (Papa Francesco)\n",
        db,
    );
}

fn print_file(machine: &mut Machine, db: &Db, file_path: &str) {
    let content = fs::read_to_string(file_path).unwrap();
    machine.print(&content, db);
}

fn start_runner(rx: UnboundedReceiver<Instruction>) {
    info!("Started worker");
    let path = env::args().nth(1).unwrap();
    let mut runner = Hal::new(&path, rx);
    runner.prepare();
    runner.run();
}

fn do_the_rest(tx: UnboundedSender<Instruction>) {
    info!("Machine is starting up");
    let mut machine = Machine::new(tx);
    let db = Db::new();

    let second_command_line_arg = env::args().nth(2);
    match second_command_line_arg {
        Some(path) => print_file(&mut machine, &db, &path),
        None => welcome(&mut machine, &db),
    };
    machine.shutdown();
}

#[tokio::main]
async fn main() {
    let mut builder = Builder::from_default_env();
    // output logs to the STDOUT
    builder.target(Target::Stdout);
    builder.init();

    let (tx, rx) = mpsc::unbounded_channel::<Instruction>();
    // start_runner(rx).await;
    // do_the_rest(tx).await;

    do_the_rest(tx);

    let _ = tokio::task::spawn_blocking(move || {
        info!("the runner is starting");
        start_runner(rx);
        info!("the runner is finished");
    })
    .await;



    // let (_first) = tokio::join!(start_runner(rx));
}
