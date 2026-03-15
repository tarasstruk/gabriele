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
    use crate::motion::Cmd;
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
