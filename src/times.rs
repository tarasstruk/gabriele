use std::thread::sleep;
use std::time::Duration;

pub const LONG_MS: u64 = 1000;
pub const SHORT_MS: u64 = 200;
pub const TINY_MS: u64 = 50;

pub fn wait_long() {
    wait(LONG_MS);
}

pub fn wait_short() {
    wait(SHORT_MS);
}

pub fn wait_tiny() {
    wait(TINY_MS);
}
pub fn wait(millis: u64) {
    sleep(Duration::from_millis(millis));
}
