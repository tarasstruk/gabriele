mod hal;

use serialport::SerialPort;

pub struct Machine {
    conn: Box<dyn SerialPort>,
}

pub trait SerialConnection {
    fn connect(path: &str) -> Box<dyn SerialPort>;
}

pub trait Commands {
    fn write_byte(&mut self, input: u8);
    fn await_acknowledge(&mut self);
    fn command(&mut self, bytes: &[u8]);
    fn prepare(&mut self);
    fn go_offline(&mut self);
    fn wait_long(&self);
    fn wait_short(&self);
    fn wait_tiny(&self);
    fn wait(&self, millis: u64);
    fn roll_forward(&mut self, steps: u8);
    fn roll_backward(&mut self, steps: u8);
    fn carriage_forward(&mut self, steps: u8);
    fn carriage_backward(&mut self, steps: u8);
}

impl Machine {
    pub fn new(path: &str) -> Self {
        let conn = Self::connect(path);
        Self { conn }
    }
}
