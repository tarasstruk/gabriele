mod hal;

use gabriele::machine::InstructionSender;
use gabriele::printing::Instruction;
pub use hal::Hal;
use tokio::sync::mpsc::UnboundedSender;

pub struct SenderWrapper(pub UnboundedSender<Instruction>);

impl InstructionSender for SenderWrapper {
    #[allow(clippy::manual_async_fn)]
    fn send(&self, instr: Instruction) -> impl core::future::Future<Output = ()> + '_ {
        async move {
            self.0.send(instr).expect("cannot send instruction");
        }
    }
}
