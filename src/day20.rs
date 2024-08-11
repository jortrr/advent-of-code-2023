mod problem;

use nom::character::complete::space1;
use problem::*;

type Name = String;
type Modules = HashMap<Name, Module>;
type Memory = HashMap<Name, PulseKind>;

#[derive(Copy, Clone, Debug)]
enum State {
    On,
    Off,
}

impl Into<PulseKind> for State {
    fn into(self) -> PulseKind {
        match self {
            State::On => PulseKind::High,
            State::Off => PulseKind::Low,
        }
    }
}

impl State {
    fn flip(&self) -> State {
        use State::*;
        match &self {
            On => Off,
            Off => On,
        }
    }
}

#[derive(Clone, Debug)]
enum ModuleKind {
    FlipFlop(State),
    Conjuction(Memory),
    Broadcaster,
    Untyped,
}

impl ModuleKind {
    fn from_char(c: char) -> ModuleKind {
        use ModuleKind::*;
        match c {
            '%' => FlipFlop(State::Off),
            '&' => Conjuction(Memory::new()),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
struct Module {
    kind: ModuleKind,
    name: Name,
    destinations: Vec<Name>,
}

impl Parse for Module {
    fn parse(input: Input) -> Self {
        Module::parse_module(&input).unwrap().1
    }
}

impl Module {
    fn parse_module(input: &str) -> IResult<&str, Module> {
        // Parsing "broadcaster" as a special case
        let broadcaster_parser = map(tag("broadcaster"), |_| Module {
            kind: ModuleKind::Broadcaster,
            name: "broadcaster".to_string(),
            destinations: Vec::new(),
        });

        // Parsing a symbolic module with a special character and a name
        let symbolic_module_parser = map(
            tuple((one_of("%&"), alpha1)),
            |(symbol, name): (char, &str)| Module {
                kind: ModuleKind::from_char(symbol),
                name: name.to_string(),
                destinations: Vec::new(),
            },
        );

        let (rest, mut module) = alt((broadcaster_parser, symbolic_module_parser))(input)?;
        let (rest, _) = tuple((space1, tag("->"), space1))(rest)?;
        let (rest, destination) = separated_list1(tag(", "), alpha1)(rest)?;
        module.destinations = destination.iter().map(|s| s.to_string()).collect();

        Ok((rest, module))
    }

    fn generate_pulses(&self, kind: PulseKind) -> Vec<Pulse> {
        self.destinations
            .iter()
            .map(|name| Pulse {
                from: self.name.clone(),
                to: name.clone(),
                kind,
            })
            .collect()
    }

    fn handle_pulse(&mut self, pulse: Pulse, system: &mut System) {
        use ModuleKind::*;
        match &mut self.kind {
            FlipFlop(state) => {
                if pulse.kind == PulseKind::Low {
                    *state = state.flip();
                    let state = *state;
                    system.enqueue_pulses(self.generate_pulses(state.into()));
                }
            }
            Conjuction(memory) => {
                memory.insert(pulse.from, pulse.kind);
                let new_pulse = if memory.values().all(|kind| *kind == PulseKind::High) {
                    PulseKind::Low
                } else {
                    PulseKind::High
                };
                system.enqueue_pulses(self.generate_pulses(new_pulse));
            }
            Broadcaster => system.enqueue_pulses(self.generate_pulses(pulse.kind)),
            Untyped => (),
            _ => unreachable!("Type: {:?}", self.kind),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum PulseKind {
    High,
    Low,
}

struct Pulse {
    from: Name,
    to: Name,
    kind: PulseKind,
}

impl Debug for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -{:?}-> {}", self.from, self.kind, self.to)
    }
}

/// A System of wired up Modules that can send pulses to eachother
struct System {
    modules: Modules,
    pulses: Queue<Pulse>,
    low_pulses: Int,
    high_pulses: Int,
}

impl Parse for System {
    fn parse(input: Input) -> Self {
        let lines: Vec<String> = InputLines::from(input).into();
        let modules: Modules = lines
            .into_iter()
            .map(Module::parse)
            .map(|m| (m.name.clone(), m))
            .collect();
        System {
            modules,
            pulses: Queue::new(),
            low_pulses: 0,
            high_pulses: 0,
        }
    }
}

impl System {
    fn enqueue_pulse(&mut self, from: &str, to: &str, kind: PulseKind) {
        self.pulses.push_back(Pulse {
            from: from.to_string(),
            to: to.to_string(),
            kind,
        });
        // Increment the correct pulse kind
        match kind {
            PulseKind::High => self.high_pulses += 1,
            PulseKind::Low => self.low_pulses += 1,
        }
    }

    fn enqueue_pulses(&mut self, pulses: Vec<Pulse>) {
        pulses
            .iter()
            .for_each(|pulse| self.enqueue_pulse(&pulse.from, &pulse.to, pulse.kind))
    }

    /// Press the button and run the System, until all pulses have been handled
    fn press_button(&mut self) {
        self.enqueue_pulse("button", "broadcaster", PulseKind::Low);
        self.run_until_all_pulses_handled();
    }

    /// Press the button and run the System, until all pulses have been handled, `times` times after each other
    fn press_button_repeatedly(&mut self, times: Int) {
        for _ in 0..times {
            self.press_button();
        }
    }

    fn run_until_all_pulses_handled(&mut self) {
        while !self.pulses.is_empty() {
            let pulse = self.pulses.pop_front().unwrap();
            //dbg!(&pulse);

            let mut destination = self
                .modules
                .get(&pulse.to)
                .unwrap_or(&Module {
                    kind: ModuleKind::Untyped,
                    name: pulse.to.clone(),
                    destinations: Vec::new(),
                })
                .clone();
            destination.handle_pulse(pulse, self);

            // Overwrite the old destination module with the possibly changed one
            self.modules.insert(destination.name.clone(), destination);
        }
    }
}

struct DayTwenty {}

impl Problem for DayTwenty {
    const YEAR: Year = 2023;
    const DAY: Day = 20;
    const PART_ONE_EXAMPLE_EXPECTED: Answer = 11687500;
    const PART_ONE_EXPECTED: Answer = 0;
    const PART_TWO_EXAMPLE_EXPECTED: Answer = 0;
    const PART_TWO_EXPECTED: Answer = 0;

    fn example_input() -> ExampleInput {
        //TODO: Split up examples, so I can have multiple
        "
        broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output
        "
    }

    fn solve_part_one(input: Input, _is_example: bool) -> Answer {
        dbg!(InputLines::from(input.clone()));
        let mut system = System::parse(input);
        system.press_button_repeatedly(1000);
        dbg!((system.high_pulses, system.low_pulses));
        system.high_pulses * system.low_pulses
    }

    fn solve_part_two(input: Input, _is_example: bool) -> Answer {
        0
    }
}

run!(DayTwenty);

/*
Expected:
button -low-> broadcaster
broadcaster -low-> a
broadcaster -low-> b
broadcaster -low-> c
a -high-> b
b -high-> c
c -high-> inv
inv -low-> a
a -low-> b
b -low-> c
c -low-> inv
inv -high-> a
*/
