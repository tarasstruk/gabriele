use log::debug;
use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};
use std::thread;
use std::time::Duration;

pub fn uart(tty_serial_unix_path: &str) -> Box<dyn SerialPort> {
    thread::sleep(Duration::from_millis(1000));
    debug!("connecting...");
    serialport::new(tty_serial_unix_path, 4800)
        .timeout(Duration::from_millis(1000))
        .flow_control(FlowControl::None)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .data_bits(DataBits::Eight)
        .open()
        .expect("failed to open the serial port")
}
