pub mod german;
use crate::gabi::printing::Instruction;
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

#[derive(PartialEq, Debug, Clone, Default)]
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
    #[allow(unused)]
    pub fn value(&self) -> u8 {
        match self {
            Self::MoveRight => 0b1000_0000,
            Self::MoveLeft => 0b1100_0000,
            Self::HoldOn => 0b0000_0000,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Symbol {
    pub idx: u8,
    pub imp: Impression,
    pub chr: char,
    pub act: ActionMapping,
    pub after: AfterSymbolPrinted,
}

impl Symbol {
    #[allow(unused)]
    pub fn new(idx: u8, chr: char) -> Self {
        Self {
            idx,
            chr,
            ..Default::default()
        }
    }

    pub fn whitespace() -> Self {
        Self {
            idx: 128,
            chr: ' ',
            imp: Default::default(),
            act: ActionMapping::Whitespace,
            after: Default::default(),
        }
    }

    pub fn cr() -> Self {
        Self {
            idx: 129,
            chr: '\n',
            imp: Default::default(),
            act: ActionMapping::CarriageReturn,
            after: Default::default(),
        }
    }

    #[allow(unused)]
    pub fn imp(mut self, impact: Impression) -> Self {
        self.imp = impact;
        self
    }

    #[allow(unused)]
    pub fn after_printed(mut self, after: AfterSymbolPrinted) -> Self {
        self.after = after;
        self
    }

    #[allow(unused)]
    pub fn mild(mut self) -> Self {
        self.imp(Impression::Mild)
    }

    #[allow(unused)]
    pub fn strong(mut self) -> Self {
        self.imp(Impression::Strong)
    }

    #[allow(unused)]
    pub fn hold(mut self) -> Self {
        self.after_printed(AfterSymbolPrinted::HoldOn)
    }

    #[allow(unused)]
    pub fn left(mut self) -> Self {
        self.after_printed(AfterSymbolPrinted::MoveLeft)
    }

    #[allow(unused)]
    pub fn instructions(&self) -> Box<dyn Iterator<Item = Instruction>> {
        let b1 = self.idx;
        let b2 = self.imp.value() | self.after.value();
        Box::new([Instruction::bytes(b1, b2)].into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gabi::printing::Instruction;

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
        let symbol = Symbol::new(81, 'ü').strong();
        let mut result = symbol.instructions();
        assert_eq!(result.next(), Some(Instruction::bytes(81, 47 + 128)));
        assert_eq!(result.next(), None);
    }

    #[test]
    fn test_instructions_with_hold_after_printed() {
        let symbol = Symbol::new(81, 'ü').hold();
        let mut result = symbol.instructions();
        assert_eq!(result.next(), Some(Instruction::bytes(81, 31 + 0)));
        assert_eq!(result.next(), None);
    }
    #[test]
    fn test_instructions_with_left_direction() {
        let symbol = Symbol::new(81, 'ü').left();
        let mut result = symbol.instructions();
        assert_eq!(result.next(), Some(Instruction::bytes(81, 31 + 128 + 64)));
        assert_eq!(result.next(), None);
    }
}
