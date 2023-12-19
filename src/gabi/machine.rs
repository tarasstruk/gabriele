use crate::action::Action;
use crate::database::Db;
use crate::gabi::position::Position;
use crate::gabi::printing::Instruction;
use anyhow::Result;
use serialport::SerialPort;
use std::default::Default;
use std::thread;
use std::time::Duration;

#[allow(unused)]
pub struct Machine {
    conn: Box<dyn SerialPort>,
    base_pos: Position,
    pos: Position,
}

impl Machine {
    pub fn new(conn: Box<dyn SerialPort>) -> Self {
        let pos: Position = Default::default();
        let base_pos = pos.clone();
        Self {
            conn,
            pos,
            base_pos,
        }
    }

    pub fn write_byte(&mut self, input: u8) {
        self.wait_tiny();
        self.conn
            .write_all(&[input])
            .expect("byte cannot be sent to machine");
    }

    pub fn await_acknowledge(&mut self) {
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

    pub fn command(&mut self, bytes: &[u8]) {
        for byte in bytes {
            println!("byte: {:0<8b}", byte);
            self.write_byte(*byte);
        }
    }

    pub fn prepare(&mut self) {
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

    pub fn go_offline(&mut self) {
        self.wait_long();
        println!("stopping accepting printing commands");
        self.command(&[0xA3, 0x00]);

        self.wait_long();
        println!("going off-line");
        self.command(&[0xA0, 0x00]);
    }

    pub fn wait_long(&self) {
        self.wait(1000);
    }

    pub fn wait_short(&self) {
        self.wait(200);
    }

    pub fn wait_tiny(&self) {
        self.wait(50);
    }

    pub fn wait(&self, millis: u64) {
        thread::sleep(Duration::from_millis(millis));
    }

    pub fn roll_forward(&mut self, steps: u16) {
        self.wait_short();
        println!("roll the paper forward by {:?}", &steps);
        let steps = steps | 0b1101_0000_0000_0000;
        // self.command(&[0b1101_0000, steps]);
        self.command(&steps.to_be_bytes());
        self.wait_long();
    }

    pub fn roll_backward(&mut self, steps: u16) {
        self.wait_short();
        println!("roll the paper backward by {:?}", &steps);
        let steps = steps | 0b1111_0000_0000_0000;
        // self.command(&[0b1111_0000, steps]);
        self.command(&steps.to_be_bytes());
        self.wait_long();
    }

    pub fn carriage_forward(&mut self, steps: u16) {
        self.wait_short();
        println!("move the carriage forward by {:?}", &steps);
        let steps = steps | 0b1100_0000_0000_0000;
        // self.command(&[0b1100_0000, steps]);
        self.command(&steps.to_be_bytes());
        self.wait_long();
    }
    pub fn carriage_backward(&mut self, steps: u16) {
        self.wait_short();
        println!("move the carriage <-backward by {:?}", &steps);
        let steps = steps | 0b1110_0000_0000_0000;
        // self.command(&[0b1110_0000, steps]);
        self.command(&steps.to_be_bytes());
        self.wait_long();
    }

    pub fn execute_printing_action(&mut self, action: Action) {
        for cmd in action.instructions() {
            match cmd {
                Instruction::SendBytes(bytes) => self.command(&bytes),
                Instruction::Idle(millis) => self.wait(millis),
            }
            self.wait_short();
            self.pos.step_right().unwrap();
        }
    }

    pub fn move_carriage(&mut self, increment: i32) -> Result<()> {
        let value = u16::try_from(increment.abs())?;
        if increment < 0 {
            self.carriage_backward(value);
        }
        if increment > 0 {
            self.carriage_forward(value);
        }
        Ok(())
    }

    pub fn move_paper(&mut self, increment: i32) -> Result<()> {
        let value = u16::try_from(increment.abs())?;
        if increment < 0 {
            self.roll_backward(value);
        }
        if increment > 0 {
            self.roll_forward(value);
        }
        Ok(())
    }

    pub fn move_relative(&mut self, increments: (i32, i32)) {
        self.move_carriage(increments.0).unwrap();
        self.move_paper(increments.1).unwrap();
    }

    pub fn execute_carriage_return(&mut self) {
        println!("--------carriage-return------------");
        println!("base position: {:?}", &self.base_pos);
        println!("current position: {:?}", &self.pos);
        let increments = self.pos.carriage_return(&self.base_pos);
        println!("Increments: {:?}", &increments);
        println!("new position: {:?}", &self.pos);
        self.move_relative(increments);
    }

    pub fn print(&mut self, input: &str, db: &Db) {
        for symbol in db.printables(input) {
            let action: Action = symbol.clone().into();
            match action {
                Action::CarriageReturn => self.execute_carriage_return(),
                _ => self.execute_printing_action(action),
            }
        }
    }
}
