pub mod german;
use std::default::Default;

#[derive(Default)]
#[allow(unused)]
#[derive(PartialEq, Debug, Clone)]
/// In order to obtain the best possible impression
/// the impression compensation should be specified
/// for each printable character individually.
/// The impression compensation range is between
/// 0b0000_0000 and 0b0011_1111 (0d63)
/// which corresponds to 5 least significant bit
/// in the second of byte of the Print-command.
/// The user has 3 pre-defined options and
/// the custom impression compensation can be specified.
/// For the custom one the range is between 0.0 and 1.0
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
    fn convert_value(rate: &f32) -> u16 {
        (rate * 63.0) as u16
    }

    #[allow(unused)]
    fn value(&self) -> u16 {
        match self {
            Self::Custom(rate) => Self::convert_value(rate),
            Self::Strongest => Self::convert_value(&1.0),
            Self::Strong => Self::convert_value(&0.75),
            Self::Normal => Self::convert_value(&0.5),
            Self::Mild => Self::convert_value(&0.25),
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
pub struct Symbol {
    pub idx: u8,
    pub imp: Impression,
    pub chr: char,
    pub act: ActionMapping,
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
        }
    }

    pub fn cr() -> Self {
        Self {
            idx: 129,
            chr: '\n',
            imp: Default::default(),
            act: ActionMapping::CarriageReturn,
        }
    }

    #[allow(unused)]
    pub fn imp(mut self, impact: Impression) -> Self {
        self.imp = impact;
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
