use crate::daisy::Symbol;

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
    fn print_symbol(&mut self, symbol: &Symbol);
}
