use super::{Commands, Machine, SerialConnection};
use crate::daisy;
use crate::daisy::{Printable, Symbol};
use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};
use std::io::Write;
use std::thread;
use std::time::Duration;

impl SerialConnection for Machine {
    fn connect(path: &str) -> Box<dyn SerialPort> {
        thread::sleep(Duration::from_millis(1000));
        println!("connecting...");
        serialport::new(path, 4800)
            .timeout(Duration::from_millis(1000))
            .flow_control(FlowControl::Hardware)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .data_bits(DataBits::Eight)
            .open()
            .expect("failed to open the serial port")
    }
}

impl Commands for Machine {
    fn write_byte(&mut self, input: u8) {
        self.wait_tiny();
        self.conn
            .write_all(&[input])
            .expect("byte cannot be sent to machine");
    }

    fn await_acknowledge(&mut self) {
        self.command(&[0xA4, 0x00]);
        for _ in 0..10 {
            self.wait_short();
            let mut buf = [0_u8];
            if let Ok(n) = self.conn.read(&mut buf) {
                println!("received byte {:?}", &buf[0]);
                if n == 1 && buf[0] == 161_u8 {
                    return;
                }
                if n == 1 && buf[0] == 160_u8 {
                    panic!("unexpected status code is received");
                }
            }
        }
        panic!("no answer is received from the machine");
    }

    fn command(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.write_byte(*byte);
        }
    }

    fn prepare(&mut self) {
        self.wait_long();
        println!("stopping accepting printing commands");
        self.command(&[0xA3, 0x00]);

        self.wait_long();
        println!("going off-line");
        self.command(&[0xA0, 0x00]);

        self.wait_long();
        println!("going first on-line");
        self.command(&[0xA1, 0x00]);

        self.wait_long();
        println!("reading the status from machine");
        self.await_acknowledge();

        self.wait_long();
        println!("preparing the machine for printing");
        self.command(&[0xA2, 0x00]);

        println!("machine is now accepting the printing commands");
        self.wait_long();
    }

    fn go_offline(&mut self) {
        self.wait_long();
        println!("stopping accepting printing commands");
        self.command(&[0xA3, 0x00]);

        self.wait_long();
        println!("going off-line");
        self.command(&[0xA0, 0x00]);
    }

    fn wait_long(&self) {
        self.wait(1000);
    }

    fn wait_short(&self) {
        self.wait(200);
    }

    fn wait_tiny(&self) {
        self.wait(50);
    }

    fn wait(&self, millis: u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    fn roll_forward(&mut self, steps: u8) {
        self.wait_short();
        println!("roll the paper forward");
        self.command(&[0b1101_0000, steps]);
        self.wait((steps as u64) * 10);
    }

    fn roll_backward(&mut self, steps: u8) {
        self.wait_short();
        println!("roll the paper backward");
        self.command(&[0b1111_0000, steps]);
        self.wait((steps as u64) * 10);
    }

    fn carriage_forward(&mut self, steps: u8) {
        self.wait_short();
        println!("move the carriage forward by {:?}", &steps);
        self.command(&[0b1100_0000, steps]);
        self.wait((steps as u64) * 10);
    }
    fn carriage_backward(&mut self, steps: u8) {
        self.wait_short();
        println!("move the carriage <-backward");
        self.command(&[0b1110_0000, steps]);
        self.wait((steps as u64) * 10);
    }
    fn print(&mut self, input: &str) {
        let things: Vec<Symbol> = daisy::printables(input, &self.db).cloned().collect();
        for thing in things {
            self.print_character(&thing);
        }
    }
    fn print_character(&mut self, printable: &impl Printable) {
        self.command(&[printable.index(), 0b1001_0110]);
        self.wait_short();
    }
}
