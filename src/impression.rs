use deku::DekuWrite;
use serde::{Deserialize, Serialize};

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
    use crate::symbol::{AfterSymbolPrinted, SymbolPrintingAttrs};
    use deku::DekuContainerWrite;

    #[test]
    fn mild() {
        let hit = SymbolPrintingAttrs {
            direction: AfterSymbolPrinted::HoldOn,
            impression: Impression::Mild,
        }
        .to_bytes()
        .unwrap()[0];
        assert_eq!(hit, 15)
    }

    #[test]
    fn normal() {
        let hit = SymbolPrintingAttrs {
            direction: AfterSymbolPrinted::HoldOn,
            impression: Impression::default(),
        }
        .to_bytes()
        .unwrap()[0];
        assert_eq!(hit, 31)
    }

    #[test]
    fn strong() {
        let hit = SymbolPrintingAttrs {
            direction: AfterSymbolPrinted::HoldOn,
            impression: Impression::Strong,
        }
        .to_bytes()
        .unwrap()[0];
        assert_eq!(hit, 47)
    }

    #[test]
    fn strongest() {
        let hit = SymbolPrintingAttrs {
            direction: AfterSymbolPrinted::HoldOn,
            impression: Impression::Strongest,
        }
        .to_bytes()
        .unwrap()[0];
        assert_eq!(hit, 63)
    }
}
