use anyhow::bail;
use gabriele::Db;
use std::fs;
use std::path::Path;

pub fn load_db_from_file(filename: &str) -> anyhow::Result<Db> {
    let path = Path::new(&filename);
    if path.exists() {
        let content = fs::read_to_string(filename)?;
        return toml::from_str(&content)
            .map_err(|_e| anyhow::anyhow!("the daisy wheel data is not valid in {}", filename));
    }
    bail!("requested daisy wheel file is not found: {}", filename)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gabriele::symbol::Symbol;
    use gabriele::to_symbols::ToSymbols;
    use gabriele::Db;

    #[test]
    fn test_load_database_from_file() {
        let res = load_db_from_file("wheels/German.toml");
        assert!(res.is_ok());
    }

    #[test]
    fn test_string_to_iterator_over_symbols() {
        let wheel =
            std::fs::read_to_string("wheels/German.toml").expect("Cannot read the wheel file");
        let db: Db = toml::from_str(&wheel).expect("Cannot deserialize the wheel file");

        let input = "Wombat";
        let mut first_iterator = input.to_symbols(&db);

        let mut second_iterator = input.to_symbols(&db);

        let sym_w_upper = Symbol::new('W').petal(50).strong();

        let sym_o = Symbol::new('o').petal(99);

        let sym_m = Symbol::new('m').petal(6);

        let value = first_iterator.next();
        assert_eq!(value, Some(sym_w_upper.clone()));

        let value = first_iterator.next();
        assert_eq!(value, Some(sym_o));

        let value = second_iterator.next();
        assert_eq!(value, Some(sym_w_upper));

        let value = first_iterator.next();
        assert_eq!(value, Some(sym_m));
    }
}
