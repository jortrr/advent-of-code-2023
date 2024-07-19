pub fn get(year: i32, day: u32) -> Vec<String> {
    let aoc_json_cache_path = format!(".aocf/cache/aoc{}_{}.json", year, day);
    let aoc_json_cache = aocf::Aoc::load_json_from(aoc_json_cache_path);
    let aoc_input = if aoc_json_cache.is_ok() {
        aoc_json_cache
            .unwrap()
            .init()
            .unwrap()
            .get_input(false)
            .unwrap()
    } else {
        aocf::Aoc::new()
            .year(Some(year))
            .day(Some(day))
            .init()
            .unwrap()
            .get_input(true)
            .unwrap()
    };
    aoc_input
        .split("\n")
        .map(str::to_string)
        .collect::<Vec<String>>()
}
