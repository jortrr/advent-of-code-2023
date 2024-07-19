use core::panic;
use std::fs::read_to_string;

use serde_json::Value;

pub fn get(year: i32, day: u32) -> Vec<String> {
    let aoc_json_cache_path = format!(".aocf/cache/aoc{}_{}.json", year, day);
    let aoc_json_cache = aocf::Aoc::load_json_from(&aoc_json_cache_path);
    if aoc_json_cache.is_ok() {
        let json_struct: Value =
            serde_json::from_str(&read_to_string(&aoc_json_cache_path).unwrap()).unwrap();

        match &json_struct["input"] {
            Value::String(s) => s.lines().map(|s| s.to_string()).collect(),
            _ => panic!(
                "Could not find String input field in json file: '{}'",
                aoc_json_cache_path
            ),
        }
    } else {
        panic!("Could not find json file: '{}'", aoc_json_cache_path);
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
}
