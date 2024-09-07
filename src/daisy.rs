use crate::machine::PrintingDirection;
use crate::printing::Instruction;
use std::default::Default;

#[derive(Default)]
#[allow(unused)]
#[derive(PartialEq, Debug, Clone)]
/// To reach the best printing quality
/// of each character on the paper,
/// the Impression value should be specified
///
/// The Impression range is 0..64
/// which corresponds to the 5 least-significant bits
/// in the 2-bytes printing command.
///
/// The User has 4 pre-defined options and
/// the custom impression value can be specified
/// as a ratio between the base (0) and maximum (63).
pub enum Impression {
    #[default]
    /// Normal impression, middle of the range
    Normal,
    /// 75% of the strongest impression
    Strong,
    /// 25% of the strongest impression
    Mild,
    /// The maximum possible impression
    Strongest,
    Custom(f32),
}

impl Impression {
    #[allow(unused)]
    fn convert_value(ratio: f32) -> u8 {
        (ratio * 63.0) as u8
    }

    #[allow(unused)]
    pub fn value(&self) -> u8 {
        match self {
            Self::Custom(ratio) => Self::convert_value(*ratio),
            Self::Strongest => Self::convert_value(1.0),
            Self::Strong => Self::convert_value(0.75),
            Self::Normal => Self::convert_value(0.5),
            Self::Mild => Self::convert_value(0.25),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Default)]
pub enum ActionMapping {
    #[default]
    Print,
    Whitespace,
    CarriageReturn,
}

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub enum AfterSymbolPrinted {
    // sets bits "7"=1 and "6"=0
    #[default]
    MoveRight,
    // sets bits "7"=1 and "6"=1
    #[allow(unused)]
    MoveLeft,
    // sets bits "7"=0 and "6"=0
    #[allow(unused)]
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
}

impl AfterSymbolPrinted {
    #[allow(unused)]
    pub fn value(&self) -> u8 {
        match self {
            Self::MoveRight => 0b1000_0000,
            Self::MoveLeft => 0b1100_0000,
            Self::HoldOn => 0b0000_0000,
        }
    }
}

/// `Sign` represents a petal on a daisy wheel with a moulded character or punctuation mark
/// and defines the printing parameters and after-printing behaviour.
/// The routine is the following:
/// 1. daisy wheel contains the entire character set, including punctuation marks;
/// 2. servo motor rotates the daisy wheel to a specific `idx` position;
/// 3. solenoid-operated hammer hits the selected petal with a force represented by `imp`;
/// 4. after the character is printed, `after` determines the behavior of carriage motor.
#[derive(PartialEq, Debug, Clone, Default)]
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

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Symbol {
    pub signs: Vec<Sign>,
    pub character: char,
    pub act: ActionMapping,
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
        let mut sign = self.signs[0].clone();
        sign.after = AfterSymbolPrinted::HoldOn;
        let mark = Sign {
            idx: 72,
            imp: Impression::Mild,
            ..Default::default()
        };
        self.signs = vec![sign, mark];
        self
    }

    /// Add an acute accent (é)
    /// example: `perché?` (Italian "why?", closed spelling)
    pub fn acute(mut self) -> Self {
        let mut sign = self.signs[0].clone();
        sign.after = AfterSymbolPrinted::HoldOn;
        let mark = Sign {
            idx: 14,
            imp: Impression::Mild,
            ..Default::default()
        };
        self.signs = vec![sign, mark];
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
            sign.after = after.clone()
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
        Box::new(items.into_iter())
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
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::printing::Instruction;

    #[test]
    fn test_strong_impression() {
        assert_eq!(Impression::Strong.value(), 47)
    }

    #[test]
    fn test_normal_impression() {
        assert_eq!(Impression::Normal.value(), 31)
    }

    #[test]
    fn test_strongest_impression() {
        assert_eq!(Impression::Strongest.value(), 63)
    }

    #[test]
    fn test_custom_impression() {
        assert_eq!(Impression::Custom(0.8).value(), 50)
    }

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
