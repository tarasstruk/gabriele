use crate::printing::{Instruction, SendBytesDetails};
use crate::times::*;
use log::{debug, info};
use serialport::SerialPort;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::UnboundedReceiver;

const DELAY_MS_AFTER_COMMAND_SENT: u64 = 50;
const DELAY_MS_AFTER_BYTE_SENT: u64 = 10;
const DELAY_MS_FOR_CTS_NEXT_READ: u64 = 5;

pub struct Hal {
    conn: Box<dyn SerialPort>,
    receiver: UnboundedReceiver<Instruction>,
    cts_control: bool,
}

impl Hal {
    pub fn new(conn: Box<dyn SerialPort>, receiver: UnboundedReceiver<Instruction>) -> Self {
        let cts_control = true;
        Hal {
            conn,
            receiver,
            cts_control,
        }
    }

    pub fn run(&mut self) {
        debug!("running the loop...");
        loop {
            match self.receiver.try_recv() {
                Ok(item) => {
                    debug!("Recv: {:?}", &item);
                    match item {
                        Instruction::Halt => break,
                        Instruction::Prepare => self.prepare(),
                        Instruction::SendBytes(details) => self.send_bytes_with_idle(details),
                        Instruction::Idle(millis) => wait(millis),
                        Instruction::Empty => continue,
                        Instruction::Shutdown => {
                            self.shutdown();
                            return;
                        }
                    }
                }
                Err(TryRecvError::Empty) => wait_short(),
                Err(TryRecvError::Disconnected) => return,
            }
        }
    }

    pub fn write_byte(&mut self, input: u8) {
        debug!("Writing byte: {}", input);
        wait(DELAY_MS_AFTER_BYTE_SENT);

        self.conn
            .write_all(&[input])
            .expect("byte cannot be sent to the machine");

        if !self.cts_control {
            debug!("cts control is disabled");
            return;
        }
        let mut cts_counter = 1000_u32;
        loop {
            debug!("cts_counter: {}", cts_counter);
            if let Ok(true) = self.conn.read_clear_to_send() {
                wait(DELAY_MS_FOR_CTS_NEXT_READ);
                break;
            }
            cts_counter -= 1;
            if cts_counter == 0 {
                panic!("CTS signal is not pulled down for 5 seconds")
            }
        }
    }

    pub fn command(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.write_byte(*byte);
        }
        wait(DELAY_MS_AFTER_COMMAND_SENT);
    }

    pub fn send_bytes_with_idle(&mut self, details: SendBytesDetails) {
        if let Some(time) = details.idle_before {
            wait(time);
        }
        for byte in details.cmd {
            self.write_byte(byte);
        }
        if let Some(time) = details.idle_after {
            wait(time);
        }
        wait(DELAY_MS_AFTER_COMMAND_SENT);
    }

    pub fn prepare(&mut self) {
        self.go_online();
        self.await_acknowledge();
        self.start_accepting_commands();
    }

    fn go_offline(&mut self) {
        info!("go off-line");
        self.write_byte(0xA0);
        self.cts_control = false;
        self.write_byte(0x00);
    }

    fn go_online(&mut self) {
        info!("go on-line");
        self.command(&[0xA1, 0x00]);
    }

    fn start_accepting_commands(&mut self) {
        info!("start accepting printing commands");
        self.command(&[0xA2, 0x00]);
        info!("machine is now accepting the commands");
    }

    fn stop_accepting_commands(&mut self) {
        info!("stop accepting printing commands");
        self.command(&[0xA3, 0x00]);
    }

    pub fn shutdown(&mut self) {
        wait_long();
        self.stop_accepting_commands();
        self.go_offline();
    }

    pub fn await_acknowledge(&mut self) {
        debug!("wait for the acknowledge");
        self.command(&[0xA4, 0x00]);
        for _ in 0..10 {
            wait_tiny();
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
