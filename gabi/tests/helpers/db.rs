use gabriele::database::Db;
use std::fs;

pub fn load_test_db() -> Db {
    let wheel = fs::read_to_string("wheels/German.toml").unwrap();
    toml::from_str(&wheel).unwrap()
}
