use env_logger::{Builder, Target};
use gabriele::database::Db;
use gabriele::hal::Hal;
use gabriele::machine::Machine;
use gabriele::printing::Instruction;
use log::{debug, info};
use std::fs;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedReceiver;

use clap::Parser;

/// Gabriele
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// path to serial port tty, example: `/dev/tty.usbserial-A10OFCFV`
    #[arg(long)]
    tty: String,

    /// optional path to a text file
    #[arg(long)]
    text: Option<String>,
}

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

fn start_runner(rx: UnboundedReceiver<Instruction>, tty_path: String) {
    info!("Started worker");
    let mut runner = Hal::new(&tty_path, rx);
    runner.prepare();
    runner.run();
}

#[tokio::main]
async fn main() {
    let mut builder = Builder::from_default_env();
    // output logs to the STDOUT
    builder.target(Target::Stdout);
    builder.init();

    let (tx, rx) = mpsc::unbounded_channel::<Instruction>();

    let args = Args::parse();

    let _ = tokio::task::spawn_blocking(move || {
        info!("the runner is starting");
        start_runner(rx, args.tty);
        info!("the runner is finished");
    });

    info!("Machine is starting up");
    let mut machine = Machine::new(tx);
    let db = Db::new();

    machine.offset(4 * 12);

    match args.text {
        Some(path) => print_file(&mut machine, &db, &path),
        None => welcome(&mut machine, &db),
    };
    machine.shutdown();
}
