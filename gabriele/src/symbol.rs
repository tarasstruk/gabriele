use crate::cmd::Impression;
use crate::machine::PrintingDirection;
use crate::printing::Instruction;
use crate::sign::Sign;
use deku::DekuWrite;

#[derive(PartialEq, Debug, Clone, Default)]
pub enum ActionMapping {
    #[default]
    Print,
    Whitespace,
    LineFeed,
}

#[derive(PartialEq, Debug, Copy, Clone, Default, DekuWrite)]
#[deku(id_type = "u8", bits = 2)]
#[deku(endian = "big")]
#[deku(ctx = "endian: deku::ctx::Endian")]
pub enum AfterSymbolPrinted {
    // sets bits "7"=1 and "6"=0
    #[default]
    #[deku(id = 0b_10)]
    MoveRight,

    // sets bits "7"=1 and "6"=1
    #[deku(id = 0b_11)]
    MoveLeft,

    // sets bits "7"=0 and "6"=0
    #[deku(id = 0b_00)]
    HoldOn,
}

#[derive(PartialEq, Debug, Clone, Default, DekuWrite, Copy)]
#[deku(endian = "big")]
#[deku(ctx = "endian: deku::ctx::Endian")]
pub struct SymbolPrintingAttrs {
    pub direction: AfterSymbolPrinted,
    pub impression: Impression,
}

#[derive(DekuWrite, PartialEq, Debug, Clone, Copy, Default)]
#[deku(endian = "big")]
#[deku(ctx = "endian: deku::ctx::Endian")]
pub struct CmdSymbol {
    #[deku(bits = 6)]
    pub code: u8,
    pub attr: SymbolPrintingAttrs,
}

impl AfterSymbolPrinted {
    fn invert(self) -> Self {
        match self {
            AfterSymbolPrinted::MoveRight => AfterSymbolPrinted::MoveLeft,
            AfterSymbolPrinted::MoveLeft => AfterSymbolPrinted::MoveRight,
            AfterSymbolPrinted::HoldOn => self,
        }
    }

    pub fn with_direction(self, dir: PrintingDirection) -> Self {
        match dir {
            PrintingDirection::Right => self,
            PrintingDirection::Left => self.invert(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Symbol {
    pub signs: [Option<Sign>; 2],
    pub character: char,
    pub act: ActionMapping,
}

impl Symbol {
    pub const fn new(character: char) -> Self {
        Self {
            character,
            signs: [None, None],
            act: ActionMapping::Print,
        }
    }

    pub fn is_groupable(&self) -> bool {
        match self.act {
            ActionMapping::Print => false,
            ActionMapping::Whitespace => true,
            ActionMapping::LineFeed => true,
        }
    }

    pub const fn petal(mut self, idx: u8) -> Self {
        let sign = Sign {
            idx,
            imp: Impression::Normal,
            after: AfterSymbolPrinted::MoveRight,
        };
        self.signs[0] = Some(sign);
        self
    }

    /// Add a grave accent (è)
    /// example: `caffè` (Italian "coffee", open spelling)
    pub const fn grave(mut self) -> Self {
        if let Some(ref mut sign) = self.signs[0] {
            sign.after = AfterSymbolPrinted::HoldOn;
            let mark = Sign {
                idx: 72,
                imp: Impression::Mild,
                after: AfterSymbolPrinted::MoveRight,
            };
            self.signs[1] = Some(mark);
        };
        self
    }

    /// Add an acute accent (é)
    /// example: `perché?` (Italian "why?", closed spelling)
    pub const fn acute(mut self) -> Self {
        if let Some(ref mut sign) = self.signs[0] {
            sign.after = AfterSymbolPrinted::HoldOn;
            let mark = Sign {
                idx: 14,
                imp: Impression::Mild,
                after: AfterSymbolPrinted::MoveRight,
            };
            self.signs[1] = Some(mark);
        };
        self
    }

    pub const fn whitespace() -> Self {
        let mut item = Self::new(' ');
        item.act = ActionMapping::Whitespace;
        item
    }

    pub const fn line_feed() -> Self {
        let mut item = Self::new('\n');
        item.act = ActionMapping::LineFeed;
        item
    }

    pub const fn imp(mut self, impression: Impression) -> Self {
        if let Some(ref mut sign) = self.signs[0] {
            sign.imp = impression
        }
        if let Some(ref mut sign) = self.signs[1] {
            sign.imp = impression
        }
        self
    }

    pub const fn mild(self) -> Self {
        self.imp(Impression::Mild)
    }

    pub const fn strong(self) -> Self {
        self.imp(Impression::Strong)
    }

    pub fn instructions(
        &self,
        direction: PrintingDirection,
    ) -> impl Iterator<Item = Instruction> + use<'_> {
        self.signs
            .iter()
            .flatten()
            .map(move |sign| sign.build_instruction(direction))
    }

    pub fn x_positions_increment(&self) -> i32 {
        let mut x = 0_i32;
        for sign in self.signs.iter().flatten() {
            match sign.after {
                AfterSymbolPrinted::MoveLeft => x -= 1,
                AfterSymbolPrinted::MoveRight => x += 1,
                _ => (),
            }
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::printing::Instruction;

    #[test]
    fn test_instructions_with_strong_impression() {
        let symbol = Symbol::new('ü').petal(81).strong();
        let mut result = symbol.instructions(Default::default());
        assert_eq!(
            result.next(),
            Some(Instruction::SendBytes(u16::from_be_bytes([81, 47 + 128])))
        );
        assert_eq!(result.next(), None);
    }

    #[test]
    fn test_instructions_with_acute_marker() {
        let symbol = Symbol::new('à').petal(94).grave();
        let mut result = symbol.instructions(Default::default());
        // 31 for Impression normal + 0 for Direction (hold)
        assert_eq!(
            result.next(),
            Some(Instruction::SendBytes(u16::from_be_bytes([94, 31])))
        );
        // 15 for Impression Mild + 128 for Direction normal
        assert_eq!(
            result.next(),
            Some(Instruction::SendBytes(u16::from_be_bytes([72, 15 + 128])))
        );
        assert_eq!(result.next(), None);
    }
}
