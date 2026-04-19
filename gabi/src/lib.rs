mod hal;

use gabriele::machine::InstructionSender;
use gabriele::printing::Instruction;
pub use hal::Hal;
use tokio::sync::mpsc::UnboundedSender;

pub struct SenderWrapper(pub UnboundedSender<Instruction>);

impl InstructionSender for SenderWrapper {
    async fn send(&self, instr: Instruction) {
        self.0.send(instr).expect("cannot send instruction");
    }
}
