use crate::symbol::Symbol;

pub trait DaisyDatabase {
    fn get(&self, character: char) -> &'static Symbol;
}

impl DaisyDatabase for &'static [Symbol] {
    fn get(&self, character: char) -> &'static Symbol {
        if let Some(sym) = self.iter().find(|symbol| symbol.character == character) {
            return sym;
        }
        panic!("Symbol not found");
    }
}

impl<T: DaisyDatabase + ?Sized> DaisyDatabase for &T {
    fn get(&self, chr: char) -> &'static Symbol {
        (*self).get(chr)
    }
}
