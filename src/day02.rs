mod problem;
use problem::*;

struct Game {
    id: Int,
    max_red: Int,
    max_green: Int,
    max_blue: Int,
}

impl Parse for Game {
    fn parse(input: Input) -> Self {
        Game::parse_game(&input).unwrap().1
    }
}

impl Game {
    /// Use nom to parse a single Game
    fn parse_game(input: &str) -> IResult<&str, Game> {
        let parse_num = |input| map_res(digit1, str::parse::<Int>)(input);
        let parse_rgb = tuple((
            parse_num,
            preceded(tag(" "), alt((tag("red"), tag("green"), tag("blue")))),
        ));

        let (rest, id) = preceded(tag("Game "), terminated(parse_num, tag(": ")))(input)?;
        let (rest, rgb) = separated_list1(tuple((one_of(",;"), tag(" "))), parse_rgb)(rest)?;

        let get_max = |color: &str| {
            rgb.iter()
                .filter(|x| x.1 == color)
                .map(|x| x.0)
                .max()
                .unwrap()
        };

        Ok((
            rest,
            Game {
                id,
                max_red: get_max("red"),
                max_green: get_max("green"),
                max_blue: get_max("blue"),
            },
        ))
    }
}

struct DayTwo {}

impl Problem for DayTwo {
    const YEAR: Year = 2023;
    const DAY: Day = 2;
    const PART_ONE_EXAMPLE_EXPECTED: Answer = -1;
    const PART_ONE_EXPECTED: Answer = 1867;
    const PART_TWO_EXAMPLE_EXPECTED: Answer = -1;
    const PART_TWO_EXPECTED: Answer = 84538;
    const RUN_EXAMPLE: bool = false;

    fn example_input() -> ExampleInput {
        "
        "
    }

    fn solve_part_one(input: Input, _is_example: bool) -> Answer {
        let input: Vec<String> = InputLines::from(input).filter_empty_lines().into();
        let solution = input
            .into_iter()
            .map(Game::parse)
            .filter(|game| game.max_red <= 12 && game.max_green <= 13 && game.max_blue <= 14)
            .map(|game| game.id)
            .sum();
        solution
    }

    fn solve_part_two(input: Input, _is_example: bool) -> Answer {
        let input: Vec<String> = InputLines::from(input).filter_empty_lines().into();
        let solution = input
            .into_iter()
            .map(Game::parse)
            .map(|game| game.max_red * game.max_green * game.max_blue)
            .sum();
        solution
    }
}

run!(DayTwo);
