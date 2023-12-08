pub mod german;
use std::default::Default;

#[derive(Default)]
#[repr(u16)]
#[allow(unused)]
#[derive(PartialEq, Debug, Clone)]
pub enum Impact {
    #[default]
    Middle = 127,
    Hard = 255,
    Soft = 90,
    Custom(u16),
}

#[derive(PartialEq, Debug, Clone, Default)]
pub enum ActionMapping {
    #[default]
    Print,
    Whitespace,
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Symbol {
    pub idx: u8,
    pub imp: Impact,
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

    #[allow(unused)]
    pub fn imp(mut self, impact: Impact) -> Self {
        self.imp = impact;
        self
    }

    #[allow(unused)]
    pub fn soft(mut self) -> Self {
        self.imp(Impact::Soft)
    }

    #[allow(unused)]
    pub fn hard(mut self) -> Self {
        self.imp(Impact::Hard)
    }
}
