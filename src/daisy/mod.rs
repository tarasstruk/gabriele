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

#[derive(PartialEq, Debug, Clone)]
pub struct Symbol {
    pub idx: u8,
    pub imp: Impact,
    pub chr: char,
}

pub enum Action {
    CarriageReturn(Option<u16>),
    Space(Option<u16>),
    PrintSymbol(Option<u16>, Symbol),
}

impl Symbol {
    #[allow(unused)]
    pub fn new(idx: u8, chr: char) -> Self {
        Self {
            idx,
            chr,
            imp: Default::default(),
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

#[allow(unused)]
pub struct Db {
    pub symbols: Box<[Symbol]>,
    pub unknown: Symbol,
}

impl Db {
    #[allow(unused)]
    pub fn get(&self, character: char) -> &Symbol {
        if let Some(result) = self.symbols.iter().find(|symbol| symbol.chr == character) {
            return result;
        }
        &(self.unknown)
    }

    fn unknown_symbol() -> Symbol {
        Symbol::new(41, '*')
    }

    pub fn new(symbols: Box<[Symbol]>) -> Self {
        let unknown = Self::unknown_symbol();
        Self { symbols, unknown }
    }

    pub fn printables<'a>(&'a self, input: &'a str) -> impl Iterator<Item = &'a Symbol> {
        input.chars().enumerate().map(move |(_, chr)| self.get(chr))
    }
}
