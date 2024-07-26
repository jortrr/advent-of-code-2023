mod macros;

type Uint = u64;

#[derive(Debug)]
struct Race {
    time: Uint,
    distance: Uint,
    wins: Vec<Race>,
}

impl Race {
    pub fn generate_wins(&mut self) {
        for i in 0..self.time {
            let time_left = self.time - i;
            let speed = i;
            let distance_travelled = speed * time_left;
            if distance_travelled > self.distance {
                self.wins.push(Race {
                    time: i,
                    distance: distance_travelled,
                    wins: Vec::new(),
                })
            }
        }
    }
}

fn main() {
    println!("AOC src/day06.rs");
    let input: Vec<String> = aoc::get(2023, 6);
    let times = input.get(0).unwrap().split_ascii_whitespace().skip(1);
    let distances = input.get(1).unwrap().split_ascii_whitespace().skip(1);
    let time_distance_tuples = times.zip(distances);

    let mut races: Vec<Race> = Vec::new();

    for (time, distance) in time_distance_tuples {
        println!("({}, {})", time, distance);
        let mut race = Race {
            time: time.to_string().parse::<Uint>().unwrap(),
            distance: distance.to_string().parse::<Uint>().unwrap(),
            wins: Vec::new(),
        };
        race.generate_wins();
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
    // Part 1
    test!(160816, number_of_ways_to_beat_record);

    //Part 2
    let convert_to_number = |s: &String| -> Uint {
        s.split_ascii_whitespace()
            .skip(1)
            .collect::<Vec<&str>>()
            .join("")
            .to_string()
            .parse::<Uint>()
            .unwrap()
    };
    let time = convert_to_number(input.get(0).unwrap());
    let distance = convert_to_number(input.get(1).unwrap());

    let mut race: Race = Race {
        time,
        distance,
        wins: Vec::new(),
    };
    race.generate_wins();
    let number_of_ways_to_beat_second_record = race.wins.len();
    test!(46561107, number_of_ways_to_beat_second_record);
}
