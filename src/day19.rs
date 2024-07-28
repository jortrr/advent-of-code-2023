use std::fmt::Debug;

mod macros;

type Int = i32;
type RuleFn = Box<dyn Fn(Part) -> Part>;

#[derive(Clone, Debug)]
struct Part {
    x: Int,
    m: Int,
    a: Int,
    s: Int,
    workflow: String,
}

impl Part {
    fn from_string(input: &String) -> Part {
        let mut i = input.split(&['{', ',', '}']).filter(|s| !s.is_empty());
        let opt_parse_int = |s: Option<&str>| s.unwrap()[2..].parse::<Int>().unwrap();
        let (x, m, a, s) = (
            opt_parse_int(i.next()),
            opt_parse_int(i.next()),
            opt_parse_int(i.next()),
            opt_parse_int(i.next()),
        );
        Part {
            x,
            m,
            a,
            s,
            workflow: "in".to_string(),
        }
    }

    fn sum_categories(&self) -> Int {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn new(name: &str) -> Workflow {
        Workflow {
            name: name.to_string(),
            rules: Vec::new(),
        }
    }

    fn from_string(input: &String) -> Workflow {
        let mut i = input.split(&['{', '}', ',']).filter(|s| !s.is_empty());
        let name = i.next().unwrap();
        let mut w = Workflow::new(name);
        w.rules = i.map(|s| Rule::from_string(s.into())).collect();
        w
    }
}

struct Rule {
    to_string: String,
    rule: RuleFn,
}

impl Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rule")
            .field("to_string", &self.to_string)
            .finish()
    }
}

impl Rule {
    fn from_string(input: String) -> Rule {
        assert!(!input.is_empty());
        if let Some(sep) = input.find(":") {
            let (c, o, v, w) = (
                input.chars().nth(0).unwrap(),
                input.chars().nth(1).unwrap(),
                input[2..sep].to_string().parse::<Int>().unwrap(),
                input[sep + 1..].to_string(),
            );

            let comparison_fn = match o {
                '>' => Box::new(move |a, b| a > b) as Box<dyn Fn(i32, i32) -> bool>,
                '<' => Box::new(move |a, b| a < b) as Box<dyn Fn(i32, i32) -> bool>,
                _ => panic!("Invalid operator: {}", o),
            };

            let rule: RuleFn = match c {
                'x' => Box::new(move |mut p: Part| {
                    if comparison_fn(p.x, v) {
                        p.workflow = w.clone();
                    }
                    p
                }),
                'm' => Box::new(move |mut p: Part| {
                    if comparison_fn(p.m, v) {
                        p.workflow = w.clone();
                    }
                    p
                }),
                'a' => Box::new(move |mut p: Part| {
                    if comparison_fn(p.a, v) {
                        p.workflow = w.clone();
                    }
                    p
                }),
                's' => Box::new(move |mut p: Part| {
                    if comparison_fn(p.s, v) {
                        p.workflow = w.clone();
                    }
                    p
                }),
                _ => panic!("Invalid category: '{}'", c),
            };

            Rule {
                to_string: input,
                rule,
            }
        } else {
            Rule {
                to_string: input.clone(),
                rule: Box::new(move |mut p: Part| {
                    p.workflow = input.clone();
                    p
                }),
            }
        }
    }

    fn apply(&self, part: &mut Part) {
        let new_part = (self.rule)(part.clone());
        if new_part.workflow != part.workflow {
            debug!(true, "{:?} + {:?} == {:?}", part, self, new_part);
        }
        *part = new_part;
    }
}

#[derive(Debug)]
struct System {
    workflows: Vec<Workflow>,
    parts: Vec<Part>,
}

impl System {
    fn new() -> System {
        System {
            workflows: Vec::new(),
            parts: Vec::new(),
        }
    }

    fn from_workflows_and_parts(input: &Vec<String>) -> System {
        let mut s = System::new();
        let mut i = input.split(|l| l.is_empty());
        let (w_input, p_input) = (i.next().unwrap(), i.next().unwrap());
        s.workflows = w_input.iter().map(|s| Workflow::from_string(s)).collect();
        s.parts = p_input.iter().map(|s| Part::from_string(s)).collect();
        s
    }

    fn process(mut self) -> System {
        for p in &mut self.parts {
            debug!(true, "{:#?}", &p);
            while !(p.workflow == "R" || p.workflow == "A") {
                debug!(false, "{:?}", &p);
                let w: &Workflow = self
                    .workflows
                    .iter()
                    .find(|w| w.name == p.workflow)
                    .unwrap();
                debug!(true, "{:?}", &w);
                for rule in &w.rules {
                    rule.apply(p);
                    if p.workflow != w.name {
                        break;
                    }
                }
            }
        }
        self
    }

    fn sum_of_accepted_parts(&self) -> Int {
        self.parts
            .iter()
            .filter(|p| p.workflow == "A")
            .map(|p| p.sum_categories())
            .sum()
    }
}

fn main() {
    println!("Hello, World! from src/day19.rs!");
    // Part 1 - Example
    let workflows_and_parts = vec_of_strings![
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
    let mut system = System::from_workflows_and_parts(&workflows_and_parts);
    dbg!(&system);
    system = system.process();
    dbg!(&system);
    test!("A", system.parts[0].workflow);
    test!("R", system.parts[1].workflow);
    test!("A", system.parts[2].workflow);
    test!("R", system.parts[3].workflow);
    test!("A", system.parts[4].workflow);
    let sum = system.sum_of_accepted_parts();
    test!(19114, sum);
}
