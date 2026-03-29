mod db_loader;
mod directive;
use directive::process_directive;

use env_logger::{Builder, Target};
use gabriele::database::{DaisyDatabase, Db};
use gabriele::hal::Hal;
use gabriele::machine::Machine;
use gabriele::printing::Instruction;
use log::{debug, info};
use std::cell::RefCell;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::ops::Deref;
use std::{fs, io};
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedReceiver;

use clap::Parser;

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

fn standard_in(machine: &mut Machine, db: RefCell<Db>) {
    debug!("Printing stdin");
    let stdin = io::stdin();
    for line in stdin.lines() {
        if let Ok(mut input) = line {
            if input.starts_with("@>") {
                process_directive(&input, db.borrow_mut());
            } else if input != *"exit" {
                input.push('\n');
                machine.print(&input, db.borrow().deref());
            } else {
                break;
            }
        } else {
            break;
        }
    }
}

fn print_file(machine: &mut Machine, db: impl DaisyDatabase, file_path: &str) {
    let content = fs::read_to_string(file_path).unwrap();
    machine.print(&content, db);
}

async fn start_runner(rx: UnboundedReceiver<Instruction>, addr: Ipv4Addr) {
    info!("Started worker");
    let addr = SocketAddr::new(IpAddr::V4(addr), 1234);
    let mut runner = Hal::new(rx, addr);
    let _ = runner.run().await;
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
        start_runner(rx, args.ip).await;
        info!("the runner is finished");
    });

    info!("Machine is starting up");
    let mut machine = Machine::new(tx);

    let wheel = fs::read_to_string("gabi/wheels/German.toml").expect("Cannot read the wheel file");
    let db: Db = toml::from_str(&wheel).expect("Cannot deserialize the wheel file");

    machine.offset(4 * 12);

    match args.text {
        Some(path) => print_file(&mut machine, &db, &path),
        None => standard_in(&mut machine, RefCell::new(db)),
    };
    machine.shutdown();
    _ = tokio::join!(handle);
}
