use env_logger::{Builder, Target};
use gabriele::connection;
use gabriele::database::Db;
use gabriele::machine::{Machine, PrintingDirection};
use log::info;
use std::env;
use std::fs;

fn welcome(machine: &mut Machine, db: &Db) {
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

/// command-line args:
/// 1. serial port path, example: /dev/tty.usbserial-A10OFCFV
/// 2. optional path to file to read
fn main() {
    let mut builder = Builder::from_default_env();
    // output logs to the STDOUT
    builder.target(Target::Stdout);
    builder.init();

    let path = env::args().nth(1).unwrap();
    let conn = connection::uart(&path);
    info!("Machine is starting up");
    let mut machine = Machine::new(conn);

    let db = Db::new();

    machine.prepare();

    let second_command_line_arg = env::args().nth(2);

    // machine.set_printing_direction(PrintingDirection::Left);

    match second_command_line_arg {
        Some(path) => print_file(&mut machine, &db, &path),
        None => welcome(&mut machine, &db),
    }

    machine.wait_long();
    machine.go_offline();
    machine.wait_short();

    // TODOs
    // debug!("homing the carriage motor");
    // machine.command(&[0b1000_0010, 0b0000_0011]);
    // debug!("homing the daisy-wheel motor");
    // machine.command(&[0b1000_0010, 0b0000_0101]);
    // debug!("homing the tape motor");
    // machine.command(&[0b1000_0010, 0b0000_1001]);
}
