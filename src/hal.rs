use crate::printing::Instruction;
use crate::times::*;
use crate::{connection, times};
use log::{debug, info};
use serialport::SerialPort;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::UnboundedReceiver;

const PREPARE_SEQUENCE_STAGE_ONE: [[u8; 2]; 3] = [
    // stopping accepting printing commands
    [0xA3, 0x00],
    // going off-line
    [0xA0, 0x00],
    // going first on-line
    [0xA1, 0x00],
];

const PREPARE_SEQUENCE_STAGE_TWO: [[u8; 2]; 1] = [
    // preparing the machine for accepting the printing commands
    [0xA2, 0x00],
];

pub struct Hal {
    conn: Box<dyn SerialPort>,
    receiver: UnboundedReceiver<Instruction>,
}

impl Hal {
    pub fn new(path: &str, receiver: UnboundedReceiver<Instruction>) -> Self {
        Hal {
            conn: connection::uart(&path),
            receiver,
        }
    }

    pub fn run(&mut self) {
        debug!("running the loop...");
        loop {
            match self.receiver.try_recv() {
                Ok(item) => {
                    debug!("Recv: {:?}", &item);
                    match item {
                        Instruction::SendBytes(bytes) => self.command(&bytes),
                        Instruction::Idle(millis) => wait(millis),
                        Instruction::Empty => continue,
                        Instruction::Shutdown => {
                            self.shutdown();
                            return;
                        }
                    }
                }
                Err(TryRecvError::Empty) => times::wait_short(),
                Err(TryRecvError::Disconnected) => return,
            }
        }
    }

    pub fn write_byte(&mut self, input: u8) {
        wait_tiny();
        self.conn
            .write_all(&[input])
            .expect("byte cannot be sent to machine");
    }

    pub fn command(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.write_byte(*byte);
        }
        wait_short();
    }

    pub fn prepare(&mut self) {
        debug!("preparing...");
        for cmd in PREPARE_SEQUENCE_STAGE_ONE {
            self.command(&cmd);
            debug!("waiting...");
            wait_long();
        }
        debug!("acknowledge...");
        self.await_acknowledge();

        debug!("last steps...");
        for cmd in PREPARE_SEQUENCE_STAGE_TWO {
            self.command(&cmd);
            wait_long();
        }

        info!("machine is now accepting the printing commands");
    }

    pub fn shutdown(&mut self) {
        wait_long();
        info!("stopping accepting printing commands");
        self.command(&[0xA3, 0x00]);
        wait_long();
        info!("going off-line");
        self.command(&[0xA0, 0x00]);
        wait_long();
    }

    pub fn await_acknowledge(&mut self) {
        // reading the status from machine
        self.command(&[0xA4, 0x00]);
        for _ in 0..10 {
            wait_short();
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
}
