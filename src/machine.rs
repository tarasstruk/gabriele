use crate::database::Db;
use crate::motion;
use crate::position::Position;
use crate::printing::{Action, Instruction};
use log::{debug, info};
use serialport::SerialPort;
use std::default::Default;
use std::thread;
use std::time::Duration;

#[allow(unused)]
pub struct Machine {
    conn: Box<dyn SerialPort>,
    base_pos: Position,
    pos: Position,
    settings: Settings,
}

#[derive(Default, Copy, Clone)]
pub struct Settings {
    pub direction: PrintingDirection,
}
#[derive(Default, Copy, Clone)]
pub enum PrintingDirection {
    #[default]
    Right,
    Left,
}

impl Machine {
    pub fn new(conn: Box<dyn SerialPort>) -> Self {
        let pos: Position = Default::default();
        let base_pos = pos.clone();
        Self {
            conn,
            pos,
            base_pos,
            settings: Default::default(),
        }
    }

    pub fn execute_instructions(&mut self, instructions: impl Iterator<Item = Instruction>) {
        for cmd in instructions {
            match cmd {
                Instruction::SendBytes(bytes) => self.send_bytes(&bytes),
                Instruction::Idle(millis) => self.idle(millis),
                Instruction::Empty => continue,
            }
        }
    }

    pub fn set_printing_direction(&mut self, dir: PrintingDirection) {
        self.settings.direction = dir;
        match self.settings.direction {
            PrintingDirection::Left => {
                // calculate the new base position
                let pos = self.base_pos.align_right();
                // update the base position
                self.base_pos = pos.clone();
                // move from the current place to the new base position
                let instructions = motion::move_absolute(self.pos.clone(), pos);
                self.execute_instructions(instructions);
                self.wait_long();
                // update the current position
                self.pos = self.base_pos.clone();
                info!("Text align: Right");
            }
            _ => info!("Text align: Left"),
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
                debug!("received byte {:?}", &buf[0]);
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
            self.write_byte(*byte);
        }
    }

    pub fn prepare(&mut self) {
        self.wait_long();
        info!("stopping accepting printing commands");
        self.command(&[0xA3, 0x00]);

        self.wait_long();
        info!("going off-line");
        self.command(&[0xA0, 0x00]);

        self.wait_long();
        info!("going first on-line");
        self.command(&[0xA1, 0x00]);

        self.wait_long();
        info!("reading the status from machine");
        self.await_acknowledge();

        self.wait_long();
        info!("preparing the machine for printing");
        self.command(&[0xA2, 0x00]);

        info!("machine is now accepting the printing commands");
        self.wait_long();
    }

    pub fn go_offline(&mut self) {
        self.wait_long();
        info!("stopping accepting printing commands");
        self.command(&[0xA3, 0x00]);

        self.wait_long();
        info!("going off-line");
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

    pub fn print(&mut self, input: &str, db: &Db) {
        for symbol in db.printables(input) {
            let action = Action::new(
                symbol.clone(),
                self.base_pos.clone(),
                self.pos.clone(),
                self.settings,
            );
            action.run(self)
        }
    }
}

pub trait InstructionRunner {
    fn send_bytes(&mut self, bytes: &[u8]);

    fn update_position(&mut self, pos: Position);

    fn idle(&self, millis: u64) {
        thread::sleep(Duration::from_millis(millis));
    }
}

impl InstructionRunner for Machine {
    fn send_bytes(&mut self, bytes: &[u8]) {
        self.idle(30);
        for byte in bytes {
            self.write_byte(*byte);
        }
    }

    fn update_position(&mut self, pos: Position) {
        self.pos = pos;
    }
}
