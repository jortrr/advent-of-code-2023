use crate::*;
use nom::character::complete::space1;

type Name = String;
type Modules = HashMap<Name, Module>;
type Memory = HashMap<Name, PulseKind>;
type RxSenders = Vec<Name>;
static DEBUG: bool = false;

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

#[derive(Clone, Debug)]
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
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum PulseKind {
    High,
    Low,
}

#[derive(Clone)]
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

#[derive(Debug, Clone)]
struct PulseRecord {
    pulse: Pulse,
    at_button_press: Int,
}

/// A System of wired up Modules that can send pulses to eachother
#[derive(Debug)]
struct System {
    modules: Modules,
    pulses: Queue<Pulse>,
    pulse_history: Vec<PulseRecord>,
    low_pulses: Int,
    high_pulses: Int,
    rx_senders: Vec<Name>,
    times_pressed: Int,
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
            pulse_history: Vec::new(),
            low_pulses: 0,
            high_pulses: 0,
            rx_senders: RxSenders::new(),
            times_pressed: 0,
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
        if to == "rx" && !self.rx_senders.contains(&from.to_string()) {
            self.rx_senders.push(from.to_string());
        }
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
        for i in 0..times {
            debug!(DEBUG, "Press button: {}", i);

            self.times_pressed += 1;
            self.press_button();
            debug!(
                DEBUG,
                "(high: {}, low: {})\n", self.high_pulses, self.low_pulses
            );
        }
    }

    fn run_until_all_pulses_handled(&mut self) {
        while !self.pulses.is_empty() {
            let pulse = self.pulses.pop_front().unwrap();
            debug!(DEBUG, &pulse);

            let mut destination = self
                .modules
                .get(&pulse.to)
                .unwrap_or(&Module {
                    kind: ModuleKind::Untyped,
                    name: pulse.to.clone(),
                    destinations: Vec::new(),
                })
                .clone();
            destination.handle_pulse(pulse.clone(), self);
            self.pulse_history.push(PulseRecord {
                pulse,
                at_button_press: self.times_pressed,
            });

            // Overwrite the old destination module with the possibly changed one
            self.modules.insert(destination.name.clone(), destination);
        }
    }

    /// Initialize all conjunctions by remembering a low pulse for each input
    fn initialize_conjunctions(mut self) -> System {
        let modules: Vec<Module> = self.modules.values().cloned().collect();
        for module in modules {
            for destination in &module.destinations {
                if let Some(dest_module) = self.modules.get_mut(destination) {
                    if let ModuleKind::Conjuction(memory) = &mut dest_module.kind {
                        memory.insert(module.name.clone(), PulseKind::Low);
                    }
                }
            }
        }
        self
    }
}

pub struct DayTwenty {}

impl Problem for DayTwenty {
    fn year(&self) -> Year {
        2023
    }
    fn day(&self) -> Day {
        20
    }
    fn expect_part_one(&self) -> Answer {
        886701120
    }
    fn expect_part_two(&self) -> Answer {
        228134431501037
    }

    define_examples! {
        (
            "
                broadcaster -> a, b, c
                %a -> b
                %b -> c
                %c -> inv
                &inv -> a
            ",
            Expect::PartOne(32000000),
        ),
        (
            "
                broadcaster -> a
                %a -> inv, con
                &inv -> b
                %b -> con
                &con -> output
            ",
            Expect::PartOne(11687500),
        )
    }

    fn solve_part_one(&self, input: Input, is_example: bool) -> Answer {
        debug!(is_example, InputLines::from(input.clone()));
        let mut system = System::parse(input).initialize_conjunctions();
        system.press_button_repeatedly(1000);
        system.high_pulses * system.low_pulses
    }

    fn solve_part_two(&self, input: Input, _is_example: bool) -> Answer {
        let mut system = System::parse(input).initialize_conjunctions();
        system.press_button_repeatedly(1000);
        assert!(system.rx_senders.len() == 1);
        let rx_sender = system.rx_senders.first().unwrap().clone();
        let rx_sender_senders: Vec<_> = system
            .modules
            .clone()
            .into_values()
            .filter(|m| m.destinations.contains(&rx_sender))
            .collect();
        dbg!(&rx_sender_senders);
        let mut high_pulses_to_rx_sender: Vec<PulseRecord> = Vec::new();
        while !rx_sender_senders.iter().all(|m| {
            high_pulses_to_rx_sender
                .iter()
                .any(|p| p.pulse.from == m.name)
        }) {
            system.times_pressed += 1;
            system.press_button();
            high_pulses_to_rx_sender.extend(
                system
                    .pulse_history
                    .iter()
                    .cloned()
                    .filter(|p| p.pulse.to == *rx_sender && p.pulse.kind == PulseKind::High),
            );
            system.pulse_history.clear();
        }

        let mut lowest_high_pulses_to_rx_sender: Vec<PulseRecord> = Vec::new();
        for record in high_pulses_to_rx_sender {
            if !lowest_high_pulses_to_rx_sender
                .iter()
                .any(|p| p.pulse.from == record.pulse.from)
            {
                lowest_high_pulses_to_rx_sender.push(record);
            }
        }

        let mut solution = 1;
        for record in &lowest_high_pulses_to_rx_sender {
            solution = lcm(solution, record.at_button_press as u64);
        }

        dbg!(lowest_high_pulses_to_rx_sender);

        solution as Answer
    }
}
