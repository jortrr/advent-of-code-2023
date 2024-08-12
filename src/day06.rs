mod problem;
use problem::*;

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

struct DaySix {}

impl Problem for DaySix {
    const YEAR: Year = 2023;
    const DAY: Day = 6;
    const PART_ONE_EXPECTED: Answer = 160816;
    const PART_TWO_EXPECTED: Answer = 46561107;

    define_examples! {
        (
            "
            Time:      7  15   30
            Distance:  9  40  200
            ",
            Expect::PartsOneAndTwo(288, 71503),
        )
    }

    fn solve_part_one(input: Input, is_example: bool) -> Answer {
        let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
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
        debug!(is_example, &races);
        let mut number_of_ways_to_beat_record = 0;
        for race in &races {
            if race.wins.len() > 0 && number_of_ways_to_beat_record == 0 {
                number_of_ways_to_beat_record = 1;
            }
            number_of_ways_to_beat_record *= race.wins.len();
        }

        number_of_ways_to_beat_record as Answer
    }

    fn solve_part_two(input: Input, _is_example: bool) -> Answer {
        let input: Vec<String> = input.lines().map(|s| s.to_string()).collect();
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
        number_of_ways_to_beat_second_record as Answer
    }
}

run!(DaySix);
