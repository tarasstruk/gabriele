use crate::symbol::Symbol;

pub trait DaisyDatabase {
    fn get(&self, character: char) -> Symbol;
}

impl DaisyDatabase for &'static [Symbol] {
    fn get(&self, character: char) -> Symbol {
        if let Some(sym) = self.iter().find(|symbol| symbol.character == character) {
            return sym.clone();
        }
        panic!("Symbol not found");
    }
}
