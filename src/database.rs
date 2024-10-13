use crate::daisy::Symbol;
use serde::{Deserialize, Serialize};

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct Db {
    pub symbols: Vec<Symbol>,
    pub unknown: Symbol,
}

impl Default for Db {
    fn default() -> Self {
        Self {
            symbols: vec![],
            unknown: Symbol::new('*').petal(41),
        }
    }
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

    pub fn new() -> Self {
        Default::default()
    }

    pub fn printables<'a>(&'a self, input: &'a str) -> impl Iterator<Item = &'a Symbol> {
        input.chars().enumerate().map(move |(_, chr)| self.get(chr))
    }
}

#[cfg(test)]
mod tests {
    use super::Db;
    use crate::daisy::Symbol;

    #[test]
    fn test_string_to_iterator_over_symbols() {
        let wheel =
            std::fs::read_to_string("wheels/German.toml").expect("Cannot read the wheel file");
        let db: Db = toml::from_str(&wheel).expect("Cannot deserialize the wheel file");

        let input = "Wombat";
        let mut first_iterator = db.printables(input);

        let mut second_iterator = db.printables(input);

        let sym_w_upper = Symbol::new('W').petal(50).strong();

        let sym_o = Symbol::new('o').petal(99);

        let sym_m = Symbol::new('m').petal(6);

        let value = first_iterator.next();
        assert_eq!(value, Some(&sym_w_upper));

        let value = first_iterator.next();
        assert_eq!(value, Some(&sym_o));

        let value = second_iterator.next();
        assert_eq!(value, Some(&sym_w_upper));

        let value = first_iterator.next();
        assert_eq!(value, Some(&sym_m));
    }
}
