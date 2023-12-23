mod connection;
mod daisy;
mod database;
mod gabi;

use crate::daisy::german;
use crate::database::Db;
use gabi::machine::Machine;
use std::fs;

fn welcome(machine: &mut Machine, db: &Db) {
    machine.print(
        "Grüß Gott!\nIch bin die Gabriele 9009.\nSchön, dass Du da bist!\n",
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
    let path = std::env::args().nth(1).unwrap();
    let conn = connection::uart(&path);
    let mut machine = Machine::new(conn);

    let db = Db::new(german::symbols());

    machine.prepare();

    let second_command_line_arg = std::env::args().nth(2);

    match second_command_line_arg {
        Some(path) => print_file(&mut machine, &db, &path),
        None => welcome(&mut machine, &db),
    }

    machine.wait_long();
    machine.go_offline();
    machine.wait_short();
    println!("Gabriele says Tschüss :)");

    // TODOs
    // println!("homing the carriage motor");
    // machine.command(&[0b1000_0010, 0b0000_0011]);
    // println!("homing the daisy-wheel motor");
    // machine.command(&[0b1000_0010, 0b0000_0101]);
    // println!("homing the tape motor");
    // machine.command(&[0b1000_0010, 0b0000_1001]);
}
