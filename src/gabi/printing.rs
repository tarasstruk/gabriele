/// The basic directive for the machine
/// Idle specifies milliseconds
/// SendBytes specifies a sequence of 2 bytes to be send over a serial port
///
#[derive(PartialEq, Debug)]
pub enum Instruction {
    #[allow(unused)]
    Idle(u64),
    SendBytes([u8; 2]),
}
