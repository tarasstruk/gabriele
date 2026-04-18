mod hal;
pub use hal::Hal;

use env_logger::{Builder, Target};
use gabriele::database::DaisyDatabase;
use gabriele::machine::Machine;
use gabriele::printing::Instruction;
use log::{debug, info};
use std::net::{Ipv4Addr, SocketAddr};
use std::{fs, io};
use tokio::sync::mpsc;

use clap::Parser;
use gabi::SenderWrapper;
use gabriele::symbol::Symbol;

/// Gabriele
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// IP address of the RP2040 controller, example: 192.168.0.11
    #[arg(long)]
    ip: Ipv4Addr,

    /// Optional path to a text file to be printed
    #[arg(long)]
    text: Option<String>,
}

fn standard_in(machine: &mut Machine<SenderWrapper>, db: impl DaisyDatabase + 'static + Clone) {
    debug!("Printing stdin");
    let stdin = io::stdin();
    for line in stdin.lines() {
        if let Ok(mut input) = line {
            if input != *"exit" {
                input.push('\n');
                machine.print(&input, db.clone());
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

fn print_file(
    machine: &mut Machine<SenderWrapper>,
    db: impl DaisyDatabase + 'static,
    file_path: &str,
) {
    let content = fs::read_to_string(file_path).unwrap();
    machine.print(&content, db);
}

#[tokio::main]
async fn main() {
    let mut builder = Builder::from_default_env();
    // output logs to the STDOUT
    builder.target(Target::Stdout);
    builder.init();

    let (tx, rx) = mpsc::unbounded_channel::<Instruction>();

    let args = Args::parse();

    let handle = tokio::task::spawn(async move {
        info!("the runner is starting");
        let addr = SocketAddr::new(args.ip.into(), 1234);
        let mut runner = Hal::new(rx, addr);
        let _ = runner.run().await;
        info!("the runner is finished");
    });

    info!("Machine is starting up");
    let mut machine = Machine::new(SenderWrapper(tx));

    let db: &'static [Symbol] = &gabriele::wheels::standard::SYMBOLS;

    machine.offset(4 * 12);

    match args.text {
        Some(path) => print_file(&mut machine, db, &path),
        None => standard_in(&mut machine, db),
    };
    machine.shutdown();
    _ = tokio::join!(handle);
}
