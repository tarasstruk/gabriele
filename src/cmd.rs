use crate::symbol::CmdSymbol;
use deku::DekuWrite;
use serde::{Deserialize, Serialize};

/// Top-level Command enum.
/// The variant is identified by two most-significant bytes.
#[derive(Debug, DekuWrite, PartialEq)]
#[deku(id_type = "u8", bits = 2)]
#[deku(endian = "big")]
pub enum Cmd {
    #[deku(id = 0b11)]
    Motion(CmdMotion),
    #[deku(id = 0b10)]
    Jump(CmdJump),
    #[deku(id = 0b00)]
    SymbolLow(CmdSymbol),
    #[deku(id = 0b01)]
    SymbolHigh(CmdSymbol),
}

/// Make a "jump" with the caret
/// in a `Plus` or `Minus` direction.
#[derive(Debug, DekuWrite, PartialEq)]
#[deku(id_type = "u16", bits = 14)]
#[deku(endian = "big")]
#[deku(ctx = "endian: deku::ctx::Endian")]
pub enum CmdJump {
    /// Caret motion from right to left, `<=`
    #[deku(id = 0b00_0100_0000_0000)]
    Minus,

    /// Caret motion from left to right, `=>`
    #[deku(id = 0b00_0011_0000_0000)]
    Plus,
}

/// Caret or paper motion direction.
#[derive(Debug, DekuWrite, PartialEq)]
#[deku(id_type = "u8", bits = 2)]
#[deku(endian = "big")]
#[deku(ctx = "endian: deku::ctx::Endian")]
pub enum CmdMotionDirection {
    /// Vertical scroll to the paper bottom (roll forward)
    #[deku(id = 0b01)]
    PlusY,

    /// Vertical scroll to the paper top (roll backward)
    #[deku(id = 0b11)]
    MinusY,

    /// Caret motion from left to right, `=>`
    #[deku(id = 0b00)]
    PlusX,

    /// Caret motion from right to left, `<=`
    #[deku(id = 0b10)]
    MinusX,
}

#[derive(Debug, DekuWrite, PartialEq)]
#[deku(endian = "big")]
#[deku(ctx = "endian: deku::ctx::Endian")]
pub struct CmdMotion {
    dir: CmdMotionDirection,
    #[deku(bits = 12)]
    value: u16,
}
impl CmdMotion {
    /// Vertical scroll to the paper bottom (roll forward)
    pub fn plus_y(value: u16) -> Self {
        Self {
            dir: CmdMotionDirection::PlusY,
            value,
        }
    }

    /// Vertical scroll to the paper top (roll backward)
    pub fn minus_y(value: u16) -> Self {
        Self {
            dir: CmdMotionDirection::MinusY,
            value,
        }
    }

    /// Caret motion from left to right, `=>`
    pub fn plus_x(value: u16) -> Self {
        Self {
            dir: CmdMotionDirection::PlusX,
            value,
        }
    }

    /// Caret motion from right to left, `<=`
    pub fn minus_x(value: u16) -> Self {
        Self {
            dir: CmdMotionDirection::MinusX,
            value,
        }
    }

    pub fn delta_x(value: i16) -> Option<Self> {
        let dir = if value < 0 {
            CmdMotionDirection::MinusX
        } else if value > 0 {
            CmdMotionDirection::PlusX
        } else {
            return None;
        };

        Some(Self {
            dir,
            value: value.unsigned_abs(),
        })
    }

    pub fn delta_y(value: i16) -> Option<Self> {
        let dir = if value < 0 {
            CmdMotionDirection::MinusY
        } else if value > 0 {
            CmdMotionDirection::PlusY
        } else {
            return None;
        };

        Some(Self {
            dir,
            value: value.unsigned_abs(),
        })
    }
}

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
#[derive(Serialize, Deserialize, Default, DekuWrite, Copy)]
#[deku(id_type = "u8", bits = 6)]
#[deku(endian = "big")]
#[deku(ctx = "endian: deku::ctx::Endian")]
pub enum Impression {
    /// 25% of the strongest impression
    #[deku(id = 15)]
    Mild,

    /// Normal impression, middle of the range
    #[deku(id = 31)]
    #[default]
    Normal,

    /// 75% of the strongest impression
    #[deku(id = 47)]
    Strong,

    /// The maximum possible impression
    #[deku(id = 63)]
    Strongest,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cmd::Cmd;
    use crate::symbol::{AfterSymbolPrinted, CmdSymbol, SymbolPrintingAttrs};
    use deku::DekuContainerWrite;

    fn hit(impression: Impression) -> u8 {
        let mut sym = CmdSymbol::default();
        sym.attr = SymbolPrintingAttrs {
            direction: AfterSymbolPrinted::HoldOn,
            impression,
        };
        Cmd::SymbolLow(sym).to_bytes().unwrap()[1]
    }

    #[test]
    fn mild() {
        assert_eq!(hit(Impression::Mild), 15)
    }

    #[test]
    fn normal() {
        assert_eq!(hit(Impression::default()), 31)
    }

    #[test]
    fn strong() {
        assert_eq!(hit(Impression::Strong), 47)
    }

    #[test]
    fn strongest() {
        assert_eq!(hit(Impression::Strongest), 63)
    }
}
