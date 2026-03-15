use deku::{DekuContainerWrite, DekuWrite};
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
#[derive(Serialize, Deserialize, Default, DekuWrite)]
#[deku(id_type = "u8", bits = 8)]
#[deku(endian = "big")]
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

impl Impression {
    pub fn value(&self) -> u8 {
        self.to_bytes().unwrap()[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mild() {
        assert_eq!(Impression::Mild.value(), 15)
    }

    #[test]
    fn test_normal() {
        assert_eq!(Impression::Normal.value(), 31)
    }

    #[test]
    fn test_strong() {
        assert_eq!(Impression::Strong.value(), 47)
    }

    #[test]
    fn test_strongest() {
        assert_eq!(Impression::Strongest.value(), 63)
    }
}
