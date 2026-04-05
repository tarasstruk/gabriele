use crate::symbol::Symbol;

pub trait DaisyDatabase {
    fn get(&self, character: char, count: usize) -> Symbol;
}

impl DaisyDatabase for &'static [Symbol] {
    fn get(&self, character: char, count: usize) -> Symbol {
        if let Some(sym) = self.iter().find(|symbol| symbol.character == character) {
            let mut sym = sym.clone();
            if count > 1 {
                sym.repeat_times.replace(count);
            }
            return sym;
        }
        panic!("Symbol not found");
    }
}
