use crate::impression::Impression;
use crate::machine::PrintingDirection;
use crate::printing::Instruction;
use crate::sign::Sign;
use itertools::repeat_n;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Default, Serialize, Deserialize)]
pub enum ActionMapping {
    #[default]
    Print,
    Whitespace,
    CarriageReturn,
}

#[derive(PartialEq, Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub enum AfterSymbolPrinted {
    // sets bits "7"=1 and "6"=0
    #[default]
    MoveRight,
    // sets bits "7"=1 and "6"=1
    MoveLeft,
    // sets bits "7"=0 and "6"=0
    HoldOn,
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

    pub fn value(&self) -> u8 {
        match self {
            Self::MoveRight => 0b1000_0000,
            Self::MoveLeft => 0b1100_0000,
            Self::HoldOn => 0b0000_0000,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Default, Serialize, Deserialize)]
pub struct Symbol {
    pub signs: Vec<Sign>,
    pub character: char,
    pub act: ActionMapping,
    pub repeat_times: Option<usize>,
}

impl Symbol {
    pub fn new(character: char) -> Self {
        Self {
            character,
            signs: Vec::with_capacity(2),
            ..Default::default()
        }
    }

    pub fn petal(mut self, idx: u8) -> Self {
        let sign = Sign {
            idx,
            ..Default::default()
        };
        self.signs.push(sign);
        self
    }

    /// Add a grave accent (è)
    /// example: `caffè` (Italian "coffee", open spelling)
    pub fn grave(mut self) -> Self {
        let sign = &mut self.signs[0];
        sign.after = AfterSymbolPrinted::HoldOn;
        let mark = Sign {
            idx: 72,
            imp: Impression::Mild,
            ..Default::default()
        };
        self.signs.push(mark);
        self
    }

    /// Add an acute accent (é)
    /// example: `perché?` (Italian "why?", closed spelling)
    pub fn acute(mut self) -> Self {
        let sign = &mut self.signs[0];
        sign.after = AfterSymbolPrinted::HoldOn;
        let mark = Sign {
            idx: 14,
            imp: Impression::Mild,
            ..Default::default()
        };
        self.signs.push(mark);
        self
    }

    pub fn whitespace() -> Self {
        let mut item = Self::new(' ');
        item.act = ActionMapping::Whitespace;
        item
    }

    pub fn cr() -> Self {
        let mut item = Self::new('\n');
        item.act = ActionMapping::CarriageReturn;
        item
    }

    pub fn imp(mut self, impression: Impression) -> Self {
        for sign in self.signs.iter_mut() {
            sign.imp = impression.clone()
        }
        self
    }

    pub fn after_printed(mut self, after: AfterSymbolPrinted) -> Self {
        for sign in self.signs.iter_mut() {
            sign.after = after
        }
        self
    }

    pub fn mild(self) -> Self {
        self.imp(Impression::Mild)
    }

    pub fn strong(self) -> Self {
        self.imp(Impression::Strong)
    }

    #[allow(unused)]
    pub fn hold(self) -> Self {
        self.after_printed(AfterSymbolPrinted::HoldOn)
    }

    #[allow(unused)]
    pub fn left(self) -> Self {
        self.after_printed(AfterSymbolPrinted::MoveLeft)
    }

    pub fn instructions(
        &self,
        direction: PrintingDirection,
    ) -> Box<dyn Iterator<Item = Instruction>> {
        let items: Vec<Instruction> = self
            .signs
            .iter()
            .map(|sign| sign.build_instruction(direction))
            .collect();
        let times = self.repeat_times.unwrap_or(1);
        info!(
            "printing {} times the character {:?}",
            times, self.character
        );
        let rep: Vec<Instruction> = repeat_n(items, times).flatten().collect();
        Box::new(rep.into_iter())
    }

    pub fn x_positions_increment(&self) -> i32 {
        let mut x = 0_i32;
        for sign in self.signs.iter() {
            match sign.after {
                AfterSymbolPrinted::HoldOn => (),
                AfterSymbolPrinted::MoveLeft => x -= 1,
                AfterSymbolPrinted::MoveRight => x += 1,
            }
        }
        x * (self.repeat_times.unwrap_or(1) as i32)
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
        assert_eq!(result.next(), Some(Instruction::bytes(81, 47 + 128)));
        assert_eq!(result.next(), None);
    }

    #[test]
    fn test_instructions_with_hold_after_printed() {
        let symbol = Symbol::new('ü').petal(81).hold();
        let mut result = symbol.instructions(Default::default());
        assert_eq!(result.next(), Some(Instruction::bytes(81, 31 + 0)));
        assert_eq!(result.next(), None);
    }
    #[test]
    fn test_instructions_with_left_direction() {
        let symbol = Symbol::new('ü').petal(81).left();
        let mut result = symbol.instructions(Default::default());
        assert_eq!(result.next(), Some(Instruction::bytes(81, 31 + 128 + 64)));
        assert_eq!(result.next(), None);
    }

    #[test]
    fn test_instructions_with_acute_marker() {
        let symbol = Symbol::new('à').petal(94).grave();
        let mut result = symbol.instructions(Default::default());
        // 31 for Impression normal + 0 for Direction (hold)
        assert_eq!(result.next(), Some(Instruction::bytes(94, 31)));
        // 15 for Impression Mild + 128 for Direction normal
        assert_eq!(result.next(), Some(Instruction::bytes(72, 15 + 128)));
        assert_eq!(result.next(), None);
    }
}
