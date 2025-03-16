use crate::impression::Impression;
use crate::machine::PrintingDirection;
use crate::printing::Instruction;
use crate::symbol::AfterSymbolPrinted;
use serde::{Deserialize, Serialize};

/// `Sign` represents a petal on a daisy wheel with a moulded character or punctuation mark
/// and defines the printing parameters and after-printing behaviour.
/// The routine is the following:
/// 1. daisy wheel contains the entire character set, including punctuation marks;
/// 2. servo motor rotates the daisy wheel to a specific `idx` position;
/// 3. solenoid-operated hammer hits the selected petal with a force represented by `imp`;
/// 4. after the character is printed, `after` determines the behavior of carriage motor.
#[derive(PartialEq, Debug, Clone, Default, Serialize, Deserialize)]
pub struct Sign {
    pub idx: u8,
    pub imp: Impression,
    pub after: AfterSymbolPrinted,
}

impl Sign {
    /// Build a single `Instruction` for the `Sign` taking ito account
    /// current `PrintingDirection`
    pub fn build_instruction(&self, dir: PrintingDirection) -> Instruction {
        let b1 = self.idx;
        let b2 = self.imp.value() | self.after.with_direction(dir).value();
        Instruction::bytes(b1, b2)
    }
}
