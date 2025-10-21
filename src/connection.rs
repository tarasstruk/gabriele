use core::time::Duration;
use log::debug;
use tokio_serial::{new, SerialPort};

pub fn uart(tty_serial_unix_path: &str) -> Box<dyn SerialPort> {
    debug!("connecting...");
    new(tty_serial_unix_path, 115200)
        .timeout(Duration::from_millis(1000))
        .open()
        .expect("failed to open the serial port")
}
