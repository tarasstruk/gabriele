use core::time::Duration;
use log::debug;
use tokio_serial::{new, SerialStream};

pub fn uart(tty_serial_unix_path: &str) -> SerialStream {
    debug!("connecting...");
    let builder = new(tty_serial_unix_path, 115200).timeout(Duration::from_millis(1000));

    SerialStream::open(&builder).unwrap()
}
