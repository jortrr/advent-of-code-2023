use std::collections::HashMap;
use std::fmt::Debug;
use std::time::Instant;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, one_of};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

mod macros;

type Int = i32;
type Workflows = HashMap<String, Workflow>;

#[derive(Clone, Debug, PartialEq)]
enum Destination {
    Accept,
    Reject,
    Workflow(String),
}

impl Destination {
    fn parse(input: &str) -> IResult<&str, Destination> {
        let (input, destination) = alt((tag("A"), tag("R"), alpha1))(input)?;
        match destination {
            "A" => Ok((input, Destination::Accept)),
            "R" => Ok((input, Destination::Reject)),
            _ => Ok((input, Destination::Workflow(destination.to_string()))),
        }
    }
}

#[derive(Clone, Debug)]
enum Condition {
    GreaterThan(char, Int),
    LessThan(char, Int),
}

impl Condition {
    fn parse(input: &str) -> IResult<&str, Condition> {
        let (input, (var, op, val)) = tuple((
            one_of("xmas"),
            one_of("<>"),
            map_res(digit1, |s: &str| s.parse()),
        ))(input)?;
        let condition = match op {
            '>' => Condition::GreaterThan(var, val),
            '<' => Condition::LessThan(var, val),
            _ => unreachable!(),
        };
        Ok((input, condition))
    }

    fn evaluate(&self, part: Part) -> bool {
        match self {
            Condition::GreaterThan(var, val) => part.get(*var) > *val,
            Condition::LessThan(var, val) => part.get(*var) < *val,
        }
    }
}

#[derive(Clone, Debug)]
enum Rule {
    Evaluation(Condition, Destination),
    Tautology(Destination),
}

impl Rule {
    fn parse(input: &str) -> IResult<&str, Rule> {
        alt((
            map(
                tuple((Condition::parse, tag(":"), Destination::parse)),
                |(c, _, d)| Rule::Evaluation(c, d),
            ),
            map(Destination::parse, Rule::Tautology),
        ))(input)
    }

    fn evaluate(&self, part: Part) -> Option<Destination> {
        match self {
            Rule::Evaluation(condition, destination) => {
                if condition.evaluate(part) {
                    Some(destination.clone())
                } else {
                    None
                }
            }
            Rule::Tautology(destination) => Some(destination.clone()),
        }
    }
}

#[derive(Clone, Debug)]
struct Part {
    x: Int,
    m: Int,
    a: Int,
    s: Int,
}

impl Part {
    fn parse(input: &str) -> IResult<&str, Part> {
        let parse_num = |input| map_res(digit1, str::parse::<i32>)(input);

        let (input, (x, m, a, s)) = tuple((
            preceded(tag("{x="), parse_num),
            preceded(tag(",m="), parse_num),
            preceded(tag(",a="), parse_num),
            preceded(tag(",s="), terminated(parse_num, tag("}"))),
        ))(input)?;

        Ok((input, Part { x, m, a, s }))
    }

    fn set(&mut self, var: char, val: Int) {
        match var {
            'x' => self.x = val,
            'm' => self.m = val,
            'a' => self.a = val,
            's' => self.s = val,
            _ => unreachable!(),
        }
    }

    fn get(&self, var: char) -> Int {
        match var {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => unreachable!(),
        }
    }

    fn process(&self, workflows: &Workflows) -> Destination {
        let mut workflow = workflows.get("in").unwrap();
        loop {
            match workflow.evaluate(self) {
                Destination::Accept => return Destination::Accept,
                Destination::Reject => return Destination::Reject,
                Destination::Workflow(next) => workflow = workflows.get(&next).unwrap(),
            }
        }
    }

    fn total_rating(&self) -> Int {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn parse(input: &str) -> IResult<&str, Workflow> {
        let (input, (name, rules)) = tuple((
            terminated(map(alpha1, |s: &str| s.to_string()), tag("{")),
            terminated(separated_list1(tag(","), Rule::parse), tag("}")),
        ))(input)?;
        Ok((input, Workflow { name, rules }))
    }

    fn evaluate(&self, part: &Part) -> Destination {
        self.rules
            .iter()
            .find_map(|r| r.evaluate(part.clone()))
            .unwrap()
    }
}

fn part_1(input: String, expected: Int, example: bool) {
    let time = Instant::now();
    let (rest, workflows) = separated_list1(tag("\n"), Workflow::parse)(&input).unwrap();
    let workflows: Workflows = workflows.into_iter().map(|w| (w.name.clone(), w)).collect();
    debug!(example, "{:#?}", &workflows);
    let (_, parts) = separated_list1(tag("\n"), Part::parse)(rest.trim()).unwrap();
    debug!(example, "{:#?}", &parts);

    let sum_total_ratings: Int = parts
        .iter()
        .filter(|p| p.process(&workflows) == Destination::Accept)
        .map(|p| p.total_rating())
        .sum();

    test!(expected, sum_total_ratings);
    println!(
        "Part 1 ({}): {:?}",
        if example { "Example" } else { "Actual" },
        time.elapsed()
    );
}

fn main() {
    // Part 1 - Example
    let input = string![
        "px{a<2006:qkq,m>2090:A,rfg}",
        "pv{a>1716:R,A}",
        "lnx{m>1548:A,A}",
        "rfg{s<537:gd,x>2440:R,A}",
        "qs{s>3448:A,lnx}",
        "qkq{x<1416:A,crn}",
        "crn{x>2662:A,R}",
        "in{s<1351:px,qqz}",
        "qqz{s>2770:qs,m<1801:hdj,R}",
        "gd{a>3333:R,R}",
        "hdj{m>838:A,pv}",
        "",
        "{x=787,m=2655,a=1222,s=2876}",
        "{x=1679,m=44,a=2067,s=496}",
        "{x=2036,m=264,a=79,s=2244}",
        "{x=2461,m=1339,a=466,s=291}",
        "{x=2127,m=1623,a=2188,s=1013} ",
    ];
    part_1(input, 19114, true);
    part_1(aoc::get_string(2023, 19), 348378, false);
}
