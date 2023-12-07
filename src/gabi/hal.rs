use crate::gabi::machine::{Connection, Machine};
use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};
use std::thread;
use std::time::Duration;

impl Connection for Machine {
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
