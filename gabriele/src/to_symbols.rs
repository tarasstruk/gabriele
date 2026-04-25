use crate::database::DaisyDatabase;
use crate::symbol::Symbol;

pub trait ToSymbols {
    fn to_symbols<'a>(
        &'a self,
        db: &'a impl DaisyDatabase,
    ) -> impl Iterator<Item = &'static Symbol> + 'a;
}

impl ToSymbols for &str {
    fn to_symbols<'a>(
        &'a self,
        db: &'a impl DaisyDatabase,
    ) -> impl Iterator<Item = &'static Symbol> + 'a {
        self.chars().map(move |chr| db.get(chr))
    }
}
