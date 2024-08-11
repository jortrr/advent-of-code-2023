use std::{collections::VecDeque, fs::read_to_string};

use serde_json::Value;

mod macros;

/// Default Integer type
pub type Int = i64;

/// Default Grid type
pub type Grid<T> = Vec<Vec<T>>;

/// Default Queue type
pub type Queue<T> = VecDeque<T>;

pub fn get(year: i32, day: u32) -> String {
    let aoc_json_cache_path = format!(".aocf/cache/aoc{}_{:02}.json", year, day);
    let aoc_json_cache = aocf::Aoc::load_json_from(&aoc_json_cache_path);
    if aoc_json_cache.is_ok() {
        let json_struct: Value =
            serde_json::from_str(&read_to_string(&aoc_json_cache_path).unwrap()).unwrap();

        match &json_struct["input"] {
            Value::String(s) => return s.clone(),
            _ => {
                debug!(
                    true,
                    "AoC json file does not contain input field: '{}'.", aoc_json_cache_path
                );
            }
        };
    }

    debug!(
        true,
        "Not a valid AoC json file: '{}'.", aoc_json_cache_path
    );
    debug!(true, "Downloading json file from adventofcode.com.");
    aocf::Aoc::new()
        .year(Some(year))
        .day(Some(day))
        .init()
        .unwrap()
        .get_input(true)
        .unwrap()
}
