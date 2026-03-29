use crate::symbol::Symbol;
use log::info;
use serde::{Deserialize, Serialize};

pub trait DaisyDatabase {
    fn get(&self, character: char, count: usize) -> Symbol;
}

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

impl DaisyDatabase for &Db {
    fn get(&self, character: char, count: usize) -> Symbol {
        if let Some(sym) = self
            .symbols
            .iter()
            .find(|symbol| symbol.character == character)
        {
            info!(
                "found character {:?} to be printed {} times",
                character, count
            );
            let mut sym = sym.clone();
            if count > 1 {
                sym.repeat_times.replace(count);
            }
            return sym;
        }
        self.unknown.clone()
    }
}

impl Db {
    pub fn new() -> Self {
        Default::default()
    }
}
