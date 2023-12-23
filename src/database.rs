use crate::daisy::Symbol;

#[allow(unused)]
pub struct Db {
    pub symbols: Box<[Symbol]>,
    pub unknown: Symbol,
}

impl Db {
    #[allow(unused)]
    pub fn get(&self, character: char) -> &Symbol {
        if let Some(result) = self
            .symbols
            .iter()
            .find(|symbol| symbol.character == character)
        {
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
