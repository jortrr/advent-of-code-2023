use std::fs::read_to_string;

use serde_json::Value;

mod macros;

pub fn get(year: i32, day: u32) -> Vec<String> {
    let aoc_json_cache_path = format!(".aocf/cache/aoc{}_{:02}.json", year, day);
    let aoc_json_cache = aocf::Aoc::load_json_from(&aoc_json_cache_path);
    if aoc_json_cache.is_ok() {
        let json_struct: Value =
            serde_json::from_str(&read_to_string(&aoc_json_cache_path).unwrap()).unwrap();

        match &json_struct["input"] {
            Value::String(s) => return s.lines().map(|s| s.to_string()).collect(),
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
        .split("\n")
        .map(str::to_string)
        .collect::<Vec<String>>()
}

pub fn grid(year: i32, day: u32) -> Vec<Vec<char>> {
    get(year, day).iter().map(|s| s.chars().collect()).collect()
}
