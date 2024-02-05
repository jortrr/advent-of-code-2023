mod regex_captures;

use regex::Regex;
use std::fmt::{self, Debug};

struct Transform {
    source_range_start: i64,
    source_range_end: i64,
    range_length: i64,
    destination_range_start: i64,
    destination_range_end: i64,
    source_to_destination_translation: i64,
}

impl Transform {
    fn new(source_range_start: i64, destination_range_start: i64, range_length: i64) -> Transform {
        let destination_range_end = destination_range_start + range_length - 1;
        let source_range_end = source_range_start + range_length - 1;
        let source_to_destination_translation = destination_range_start - source_range_start;
        assert!(source_range_start + source_to_destination_translation == destination_range_start);
        assert!(source_range_end + source_to_destination_translation == destination_range_end);
        Transform {
            source_range_start,
            source_range_end,
            range_length,
            destination_range_start,
            destination_range_end,
            source_to_destination_translation,
        }
    }
}

impl fmt::Debug for Transform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "[{}, {}] -[{}]-> [{}, {}] ({})",
            self.source_range_start,
            self.source_range_end,
            self.range_length,
            self.destination_range_start,
            self.destination_range_end,
            self.source_to_destination_translation
        ))
    }
}

#[derive(Debug)]
struct Map {
    from: String,
    to: String,
    transforms: Vec<Transform>,
}

impl Map {
    fn new(from: String, to: String) -> Map {
        Map {
            from,
            to,
            transforms: Vec::new(),
        }
    }

    fn translate(&self, source: i64) -> i64 {
        for transform in &self.transforms {
            if source >= transform.source_range_start && source <= transform.source_range_end {
                //dbg!(transform);
                return source + transform.source_to_destination_translation;
            }
        }
        return source;
    }
}

fn main() {
    println!("Hello, World! from src/day05.rs!");
    let run_example = false;
    let lines = aoc_input::get(2023, 5);
    let mut input = lines.join("\n");
    if run_example {
        input = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48
    
soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
            .to_string();
    }
    println!("input:\n[{}]", input);

    let seeds_pattern = Regex::new(r"seeds:((?:\s\d+)+)").unwrap();
    let seeds_string = seeds_pattern
        .captures(&input)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    let seeds: Vec<i64> = seeds_string
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    dbg!(&seeds);

    let mut almanac: Vec<Map> = Vec::new();
    let map_pattern = Regex::new(r"(\w+)-to-(\w+)\smap:\n((?:\d+\s+\d+\s+\d+\n*)*)").unwrap();
    let map_line_pattern = Regex::new(r"(\d+)\s(\d+)\s(\d+)").unwrap();
    let m = map_pattern.captures_iter(&input);
    for i in m {
        //println!();
        let from = i.get(1).unwrap().as_str();
        let to = i.get(2).unwrap().as_str();
        //dbg!(from);
        //dbg!(to);
        let mut map = Map::new(from.to_string(), to.to_string());
        let map_lines = i.get(3).unwrap().as_str();
        //println!("{}", map);
        let lines = map_line_pattern.captures_iter(map_lines);
        for line in lines {
            let destination_range_start: i64 = line.get(1).unwrap().as_str().parse().unwrap();
            let source_range_start: i64 = line.get(2).unwrap().as_str().parse().unwrap();
            let range_length: i64 = line.get(3).unwrap().as_str().parse().unwrap();
            let transform =
                Transform::new(source_range_start, destination_range_start, range_length);
            //dbg!(&transform);
            map.transforms.push(transform);
        }
        almanac.push(map);
    }
    println!();
    dbg!(&almanac);

    let mut locations: Vec<i64> = Vec::new();
    for seed in &seeds {
        let location = get_location_from_seed(*seed, &almanac);
        locations.push(location);
    }
    locations.sort();
    println!();
    dbg!(&locations);
    let lowest_location = locations.get(0).unwrap();
    println!();
    println!("Part 1: The lowest location number is {}", lowest_location);

    let mut ranged_seeds: Vec<i64> = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        let start = seeds[i];
        let range = seeds[i + 1];
        let end = start + range - 1;
        dbg!(start, range, end);
        let start_location = get_location_from_seed(start, &almanac);
        let end_location = get_location_from_seed(end, &almanac);
        dbg!(start_location, end_location);
        assert!(start_location + range - 1 == end_location);
    }

    let mut locations: Vec<i64> = Vec::new();
    for seed in &ranged_seeds {
        let location = get_location_from_seed(*seed, &almanac);
        locations.push(location);
    }
    locations.sort();
    let lowest_location = locations.get(0).unwrap();
    println!("Part 2: The lowest location number is {}", lowest_location);
}

fn get_location_from_seed(seed: i64, almanac: &Vec<Map>) -> i64 {
    //println!();
    let mut from = "seed";
    let mut from_value: i64 = seed;
    //println!("{} : {}", from, from_value);
    while from != "location" {
        for map in almanac {
            if map.from == from {
                let to = &map.to;
                let to_value = map.translate(from_value);
                //println!("{} : {}", to, to_value);
                from = to;
                from_value = to_value;
                break;
            }
        }
    }
    return from_value;
}
