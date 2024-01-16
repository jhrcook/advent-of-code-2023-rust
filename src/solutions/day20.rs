use crate::data::load;
use crate::math_utils;
use std::collections::{HashMap, HashSet, VecDeque};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleErr {
    #[error("Input parsing error")]
    ParseInputError,
    #[error("Runtime error")]
    RuntimeError,
    #[error("An expectation required for Part 2 was violated")]
    Part2ExpectationViolated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
struct PulseMsg {
    from: String,
    to: String,
    pulse: Pulse,
}

trait Receiver {
    fn receive(&mut self, in_pulse: &PulseMsg) -> Option<VecDeque<PulseMsg>>;
}

#[derive(Debug, Clone)]
struct Broadcast {
    name: String,
    receivers: Vec<String>,
}

fn _parse_after_arrow(line: &str) -> Vec<String> {
    line.trim()
        .split("->")
        .nth(1)
        .unwrap()
        .trim()
        .split(", ")
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
}

impl Broadcast {
    fn new(receivers: &[String]) -> Self {
        Broadcast {
            name: "broadcaster".to_string(),
            receivers: receivers.to_owned(),
        }
    }

    fn from(line: &str) -> Self {
        let receivers = _parse_after_arrow(line);
        Self::new(&receivers)
    }
}

impl Receiver for Broadcast {
    fn receive(&mut self, in_pulse: &PulseMsg) -> Option<VecDeque<PulseMsg>> {
        Option::Some(
            self.receivers
                .iter()
                .map(|r| PulseMsg {
                    from: self.name.clone(),
                    to: r.clone(),
                    pulse: in_pulse.pulse,
                })
                .collect::<VecDeque<_>>(),
        )
    }
}

#[derive(Debug, Clone)]
struct Conjunction {
    name: String,
    memory: HashMap<String, Pulse>,
    receivers: Vec<String>,
}

impl Conjunction {
    fn new(name: &str, receivers: &[String]) -> Self {
        Self {
            name: name.to_string(),
            memory: HashMap::new(),
            receivers: receivers.to_owned(),
        }
    }

    fn from(line: &str) -> Self {
        let name = line.split("->").nth(0).unwrap().replace('&', "");
        let receivers = _parse_after_arrow(line);
        Self::new(name.trim(), &receivers)
    }

    fn add_input(&mut self, new_input: &str) {
        self.memory.insert(new_input.to_string(), Pulse::Low);
    }

    fn add_inputs(&mut self, new_inputs: &HashSet<&str>) {
        new_inputs.iter().for_each(|i| self.add_input(i))
    }
}

impl Receiver for Conjunction {
    fn receive(&mut self, in_pulse: &PulseMsg) -> Option<VecDeque<PulseMsg>> {
        self.memory.insert(in_pulse.from.clone(), in_pulse.pulse);
        let out_pulse = match self.memory.values().all(|p| p == &Pulse::High) {
            true => Pulse::Low,
            false => Pulse::High,
        };
        Option::Some(
            self.receivers
                .iter()
                .map(|r| PulseMsg {
                    from: self.name.clone(),
                    to: r.clone(),
                    pulse: out_pulse,
                })
                .collect::<VecDeque<_>>(),
        )
    }
}

#[derive(Debug, Clone)]
struct FlipFlop {
    name: String,
    state: bool, // true = "on", false = "off"
    receivers: Vec<String>,
}

impl FlipFlop {
    fn new(name: &str, receivers: &[String]) -> Self {
        Self {
            name: name.to_string(),
            state: false,
            receivers: receivers.to_owned(),
        }
    }

    fn from(line: &str) -> Self {
        let name = line.split("->").nth(0).unwrap().replace('%', "");
        let receivers = _parse_after_arrow(line);
        Self::new(name.trim(), &receivers)
    }
}

impl Receiver for FlipFlop {
    fn receive(&mut self, in_pulse: &PulseMsg) -> Option<VecDeque<PulseMsg>> {
        log::trace!(
            "FlipFlip {} received {:?} pulse.",
            self.name,
            in_pulse.pulse
        );
        if in_pulse.pulse == Pulse::High {
            return None;
        }
        let out_pulse = match self.state {
            false => Pulse::High,
            true => Pulse::Low,
        };
        self.state = !self.state;
        Option::Some(
            self.receivers
                .iter()
                .map(|r| PulseMsg {
                    from: self.name.clone(),
                    to: r.clone(),
                    pulse: out_pulse,
                })
                .collect::<VecDeque<_>>(),
        )
    }
}

#[derive(Debug, Clone)]
struct Output {
    name: String,
}

impl Output {
    fn new() -> Self {
        Self {
            name: "output".to_string(),
        }
    }
}

impl Receiver for Output {
    fn receive(&mut self, _: &PulseMsg) -> Option<VecDeque<PulseMsg>> {
        Option::None
    }
}

#[derive(Debug, Clone)]
enum Module {
    B(Broadcast),
    C(Conjunction),
    F(FlipFlop),
    O(Output),
}

fn _parse_input_line(line: &str) -> Result<Module, PuzzleErr> {
    if line.starts_with("broadcaster") {
        Ok(Module::B(Broadcast::from(line)))
    } else if line.starts_with('%') {
        Ok(Module::F(FlipFlop::from(line)))
    } else if line.starts_with('&') {
        Ok(Module::C(Conjunction::from(line)))
    } else {
        Err(PuzzleErr::ParseInputError)
    }
}

fn parse_input(input: &str) -> Result<HashMap<String, Module>, PuzzleErr> {
    // Parse the individual modules defined on each line.
    let mut modules = input
        .trim()
        .lines()
        .map(_parse_input_line)
        .collect::<Result<Vec<_>, PuzzleErr>>()?;

    // Manually add the `Output` module.
    modules.push(Module::O(Output::new()));

    // Convert the vector into a dictionary.
    let mut mapping = modules
        .into_iter()
        .map(|m| {
            let x: (String, Module) = match m {
                Module::B(ref a) => (a.name.clone(), m),
                Module::C(ref a) => (a.name.clone(), m),
                Module::F(ref a) => (a.name.clone(), m),
                Module::O(ref a) => (a.name.clone(), m),
            };
            x
        })
        .collect::<HashMap<String, Module>>();

    // Get inputs for Conjugation modules.
    let mut receiver_connections = HashMap::<String, HashSet<&str>>::new();
    let duplicate_mapping = mapping.clone();
    for (name, module) in duplicate_mapping.iter() {
        let recievers = match module {
            Module::B(b) => b.receivers.clone(),
            Module::F(f) => f.receivers.clone(),
            Module::C(c) => c.receivers.clone(),
            _ => Vec::new(),
        };
        recievers.iter().for_each(|r| {
            receiver_connections
                .entry(r.to_string())
                .and_modify(|s| {
                    s.insert(name.as_str());
                })
                .or_insert(HashSet::from_iter([name.as_str()]));
        });
    }
    for (receiver, input_mods) in receiver_connections.iter() {
        if let Some(Module::C(c)) = mapping.get_mut(receiver) {
            c.add_inputs(input_mods)
        }
    }
    Ok(mapping)
}

struct PulseCounter {
    low: u32,
    high: u32,
}

impl PulseCounter {
    fn new() -> Self {
        Self { low: 0, high: 0 }
    }

    fn track(&mut self, pulse_msg: &PulseMsg) {
        match pulse_msg.pulse {
            Pulse::Low => self.low += 1,
            Pulse::High => self.high += 1,
        }
    }
}

pub fn puzzle_1(input: &str, n_button_presses: u32) -> Result<u32, PuzzleErr> {
    // Parse modules from input.
    let mut modules = parse_input(input)?;

    // Tracker for the total number of pulses.
    let mut pulse_counter = PulseCounter::new();

    // Perform button presses.
    for _ in 0..n_button_presses {
        let mut pulses = VecDeque::from_iter([PulseMsg {
            from: "button".to_string(),
            to: "broadcaster".to_string(),
            pulse: Pulse::Low,
        }]);
        pulse_counter.low += 1;
        while !pulses.is_empty() {
            let pulse = pulses.pop_front().unwrap();
            log::trace!("PULSE: {:?}", pulse);
            if let Some(response) = match modules.get_mut(&pulse.to) {
                Some(Module::B(b)) => b.receive(&pulse),
                Some(Module::C(c)) => c.receive(&pulse),
                Some(Module::F(f)) => f.receive(&pulse),
                Some(Module::O(o)) => o.receive(&pulse),
                None => None,
            } {
                log::trace!("Received {} responses.", response.len());
                response.into_iter().for_each(|r| {
                    log::trace!("RESPONSE: {:?}", r);
                    pulse_counter.track(&r);
                    pulses.push_back(r);
                });
            }
        }
    }

    log::info!(
        "Final counts: {} low, {} high",
        pulse_counter.low,
        pulse_counter.high
    );
    Ok(pulse_counter.low * pulse_counter.high)
}

pub fn puzzle_2(input: &str) -> Result<u64, PuzzleErr> {
    // Parse modules from input.
    let mut modules = parse_input(input)?;

    // Get the input module for "rx" module.
    let rx_input = modules
        .values()
        .filter(|m| match m {
            Module::C(c) => c.receivers.contains(&"rx".to_string()),
            _ => false,
        })
        .collect::<Vec<_>>()
        .first()
        .cloned()
        .unwrap();
    log::info!("'rx' module input: {:?}", rx_input);

    // Dict for the memory inputs of the "rx" input.
    // Will count how many button presses until set "HIGH".
    let rx_input_inputs = match rx_input {
        Module::C(c) => Ok(c.memory.keys().cloned().collect::<HashSet<_>>()),
        _ => Err(PuzzleErr::Part2ExpectationViolated),
    }?;
    log::info!("inputs to 'rx' input: {:?}", rx_input_inputs);
    let mut rx_input_presses = HashMap::<String, u32>::new();

    for num_presses in 1..u32::MAX {
        let mut pulses = VecDeque::from_iter([PulseMsg {
            from: "button".to_string(),
            to: "broadcaster".to_string(),
            pulse: Pulse::Low,
        }]);
        while !pulses.is_empty() {
            let pulse = pulses.pop_front().unwrap();

            // Record num. button presses for HIGH pulses from inputs to input of "rx".
            if (pulse.pulse == Pulse::High)
                & rx_input_inputs.contains(&pulse.from)
                & !rx_input_presses.contains_key(&pulse.from)
            {
                log::info!(
                    "Recording {} presses for module {}",
                    num_presses,
                    pulse.from
                );
                rx_input_presses.insert(pulse.from.clone(), num_presses);
            }

            // All inputs to the input for "rx" found a HIGH pulse.
            if rx_input_inputs
                .iter()
                .all(|i| rx_input_presses.contains_key(i))
            {
                log::info!(
                    "Found button presses for all 'rx' input inputs: {:?}.",
                    rx_input_presses
                );
                return Ok(math_utils::lcm(
                    rx_input_presses
                        .values()
                        .map(|x| *x as u64)
                        .collect::<Vec<_>>(),
                ));
            }

            // Send pulse and add responses to queue.
            if let Some(response) = match modules.get_mut(&pulse.to) {
                Some(Module::B(b)) => b.receive(&pulse),
                Some(Module::C(c)) => c.receive(&pulse),
                Some(Module::F(f)) => f.receive(&pulse),
                Some(Module::O(o)) => o.receive(&pulse),
                None => None,
            } {
                response.into_iter().for_each(|r| pulses.push_back(r));
            }
        }
    }
    unreachable!();
}

pub fn main(data_dir: &str) {
    println!("Day 20: Pulse Propagation");
    let data = load(data_dir, 20, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data, 1000);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("No solution to puzzle 1: {}.", e),
    }
    assert_eq!(answer_1, Ok(944750144));

    // Puzzle 2.
    let answer_2 = puzzle_2(&data);
    match answer_2 {
        Ok(x) => println!(" Puzzle 2: {}", x),
        Err(e) => panic!("No solution to puzzle 2: {}", e),
    }
    assert_eq!(answer_2, Ok(222718819437131))
}
