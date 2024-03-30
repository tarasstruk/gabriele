use crate::connection;
use crate::printing::Instruction;
use crate::times::*;
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

    pub async fn run(&mut self) {
        debug!("running the loop...");
        loop {
            match self.receiver.try_recv() {
                Ok(item) => match item {
                    Instruction::SendBytes(bytes) => self.command(&bytes).await,
                    Instruction::Idle(millis) => wait(millis).await,
                    Instruction::Empty => continue,
                    Instruction::Shutdown => {
                        self.shutdown().await;
                        return;
                    }
                },
                Err(TryRecvError::Empty) => continue,
                Err(TryRecvError::Disconnected) => return,
            }
        }
    }

    pub async fn write_byte(&mut self, input: u8) {
        debug!("wait before writing bytes {:?}", &input);
        wait_tiny().await;
        debug!("write bytes {:?}", &input);
        self.conn
            .write_all(&[input])
            .expect("byte cannot be sent to machine");
    }

    pub async fn command(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.write_byte(*byte).await;
        }
        wait_short().await;
    }

    pub async fn prepare(&mut self) {
        debug!("preparing...");
        for cmd in PREPARE_SEQUENCE_STAGE_ONE {
            self.command(&cmd).await;
            debug!("waiting...");
            wait_long().await;
        }
        debug!("acknowledge...");
        self.await_acknowledge().await;

        debug!("last steps...");
        for cmd in PREPARE_SEQUENCE_STAGE_TWO {
            self.command(&cmd).await;
            wait_long().await;
        }

        info!("machine is now accepting the printing commands");
    }

    pub async fn shutdown(&mut self) {
        wait_long().await;
        info!("stopping accepting printing commands");
        self.command(&[0xA3, 0x00]).await;
        wait_long().await;
        info!("going off-line");
        self.command(&[0xA0, 0x00]).await;
        wait_long().await;
    }

    pub async fn await_acknowledge(&mut self) {
        // reading the status from machine
        self.command(&[0xA4, 0x00]).await;
        for _ in 0..10 {
            wait_short().await;
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
