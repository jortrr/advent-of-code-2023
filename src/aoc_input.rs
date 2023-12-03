use aocf::Aoc;

pub fn get(year: i32, day: u32) -> Vec<String> {
    Aoc::new()
        .year(Some(year))
        .day(Some(day))
        .init()
        .unwrap()
        .get_input(false)
        .unwrap()
        .split("\n")
        .map(str::to_string)
        .collect::<Vec<String>>()
}
