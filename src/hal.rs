use crate::printing::Instruction;
use crate::times::*;
use log::{debug, info};
use serialport::SerialPort;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::UnboundedReceiver;

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
                        Instruction::SendBytes(bytes) => self.command(&bytes),
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
        wait(10);

        self.conn
            .write_all(&[input])
            .expect("byte cannot be sent to the machine");

        let mut counter = 10_u32;
        loop {
            let ri = self.conn.read_ring_indicator().unwrap();
            debug!("Ring Indicator state: {:?}", ri);
            if ri {
                if !self.cts_control {
                    debug!("cts control is disabled");
                    return;
                }
                let mut cts_counter = 10_u32;
                loop {
                    wait(2);
                    if let Ok(true) = self.conn.read_clear_to_send() {
                        wait(2);
                        break;
                    }
                    cts_counter -= 1;
                    debug!("cts_counter: {}", cts_counter);
                    if cts_counter == 0 {
                        panic!("no cts signal")
                    }
                }
                break;
            }
            counter -= 1;
            if counter == 0 {
                panic!("no acknowledge signal received")
            }
            wait(2);
        }
    }

    pub fn command(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.write_byte(*byte);
        }
        wait(50);
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
