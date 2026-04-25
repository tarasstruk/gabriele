mod hal;
pub use hal::Hal;

use env_logger::{Builder, Target};
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

async fn standard_in(machine: &mut Machine<SenderWrapper, &'static [Symbol]>) {
    debug!("Printing stdin");
    let stdin = io::stdin();
    for line in stdin.lines() {
        if let Ok(mut input) = line {
            if input != *"exit" {
                input.push('\n');
                machine.print(&input).await;
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

async fn print_file(machine: &mut Machine<SenderWrapper, &'static [Symbol]>, file_path: &str) {
    let content = fs::read_to_string(file_path).unwrap();
    machine.print(&content).await;
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
    let db: &'static [Symbol] = &gabriele::wheels::standard::SYMBOLS;
    let mut machine = Machine::new(SenderWrapper(tx), db);

    machine.offset(4 * 12).await;

    match args.text {
        Some(path) => print_file(&mut machine, &path).await,
        None => standard_in(&mut machine).await,
    };

    machine.shutdown().await;
    _ = tokio::join!(handle);
}
