use env_logger::{Builder, Target};
use gabriele::database::Db;
use gabriele::hal::Hal;
use gabriele::machine::Machine;
use gabriele::printing::Instruction;
use log::{debug, info};
use std::{fs, io};
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

fn standard_in(machine: &mut Machine, db: &Db) {
    debug!("Printing stdin");
    let stdin = io::stdin();
    for line in stdin.lines() {
        if let Ok(mut input) = line {
            if input != String::from("exit") {
                input.push('\n');
                machine.print(&input, db)
            } else {
                break;
            }
        } else {
            break;
        }
    }
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
        None => standard_in(&mut machine, &db),
    };
    machine.shutdown();
}
