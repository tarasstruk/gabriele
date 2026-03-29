use crate::cmd::{Cmd, Impression};
use crate::machine::PrintingDirection;
use crate::printing::Instruction;
use crate::symbol::{AfterSymbolPrinted, CmdSymbol, SymbolPrintingAttrs};
use deku::DekuContainerWrite;
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
        let attr = SymbolPrintingAttrs {
            direction: self.after.with_direction(dir),
            impression: self.imp,
        };

        if b1 > 100 {
            panic!("not a valid petal index")
        }

        let cmd = if b1 > 0x3f {
            Cmd::SymbolHigh(CmdSymbol {
                code: b1 & 0x3f,
                attr,
            })
        } else {
            Cmd::SymbolLow(CmdSymbol { code: b1, attr })
        };

        let out = cmd.to_bytes().unwrap();
        Instruction::SendBytes(u16::from_be_bytes([out[0], out[1]]))
    }
}
