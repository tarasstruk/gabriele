use std::thread::sleep;
use std::time::Duration;

pub fn wait_long() {
    wait(1000);
}

pub fn wait_short() {
    wait(200);
}

pub fn wait_tiny() {
    wait(50);
}
pub fn wait(millis: u64) {
    sleep(Duration::from_millis(millis));
}
