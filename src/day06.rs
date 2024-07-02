#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
    wins: Vec<Race>,
}

fn main() {
    println!("Hello, World! from src/day06.rs!");
    let input: Vec<String> = aoc_input::get(2023, 6);
    let times = input.get(0).unwrap().split_ascii_whitespace().skip(1);
    let distances = input.get(1).unwrap().split_ascii_whitespace().skip(1);
    let time_distance_tuples = times.zip(distances);

    let mut races: Vec<Race> = Vec::new();

    for (time, distance) in time_distance_tuples {
        println!("({}, {})", time, distance);
        let mut race = Race {
            time: time.to_string().parse::<u32>().unwrap(),
            distance: distance.to_string().parse::<u32>().unwrap(),
            wins: Vec::new(),
        };
        for i in 0..race.time {
            let time_left = race.time - i;
            let speed = i;
            let distance_travelled = speed * time_left;
            if distance_travelled > race.distance {
                race.wins.push(Race {
                    time: i,
                    distance: distance_travelled,
                    wins: Vec::new(),
                })
            }
        }
        races.push(race);
    }
    dbg!(&races);
    let mut number_of_ways_to_beat_record = 0;
    for race in &races {
        if race.wins.len() > 0 && number_of_ways_to_beat_record == 0 {
            number_of_ways_to_beat_record = 1;
        }
        number_of_ways_to_beat_record *= race.wins.len();
    }
    dbg!(number_of_ways_to_beat_record);
}
