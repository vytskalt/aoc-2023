use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Module<'a> {
    module_type: ModuleType<'a>,
    connections: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn parse(line: &'a str) -> (&'a str, Self) {
        let mut parts = line.split(" -> ");
        let (name, module_type) = ModuleType::read_name(parts.next().unwrap());
        let connections = parts.next().unwrap().split(", ").collect::<Vec<_>>();
        (
            name,
            Self {
                module_type,
                connections,
            },
        )
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum ModuleType<'a> {
    Broadcaster,
    FlipFlop { is_on: bool },
    Conjunction { low: Vec<&'a str> },
}

impl<'a> ModuleType<'a> {
    fn read_name(name: &'a str) -> (&'a str, Self) {
        match name.chars().nth(0) {
            Some('%') => (&name[1..], Self::FlipFlop { is_on: false }),
            Some('&') => (&name[1..], Self::Conjunction { low: vec![] }),
            _ if name == "broadcaster" => (name, ModuleType::Broadcaster),
            _ => panic!("Unknown module type; name = {name}"),
        }
    }

    fn on_recv(&mut self, sender: &'a str, receiving_pulse: Pulse) -> Option<Pulse> {
        match self {
            Self::Broadcaster => {
                return Some(receiving_pulse);
            }
            Self::FlipFlop { is_on } if matches!(receiving_pulse, Pulse::Low) => {
                let pulse = if *is_on {
                    *is_on = false;
                    Pulse::Low
                } else {
                    *is_on = true;
                    Pulse::High
                };
                return Some(pulse);
            }
            Self::Conjunction { low } => {
                match receiving_pulse {
                    Pulse::Low => {
                        if !low.contains(&sender) {
                            low.push(sender);
                        }
                    }
                    Pulse::High => {
                        if let Some(pos) = low.iter().position(|x| *x == sender) {
                            low.remove(pos);
                        }
                    }
                }

                let pulse = if low.is_empty() {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                return Some(pulse);
            }
            _ => {}
        }
        None
    }
}

#[derive(Debug, Clone, Copy)]
enum Pulse {
    Low,
    High,
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn process<'a>(input: &'a str) -> u64 {
    let mut modules = input.lines().map(Module::parse).collect::<HashMap<_, _>>();
    let mut to_add: Vec<(&'a str, &'a str)> = vec![];
    for (name, module) in &modules {
        for conn_name in &module.connections {
            to_add.push((name, conn_name));
        }
    }

    for (low_name, module_name) in to_add {
        let Some(module) = modules.get_mut(module_name) else {
            continue;
        };
        if let ModuleType::Conjunction { ref mut low } = module.module_type {
            low.push(&low_name);
        };
    }

    let (upper_name, upper) = modules
        .iter()
        .find(|(_, m)| m.connections.contains(&("rx")))
        .unwrap();

    let ModuleType::Conjunction { ref low } = upper.module_type else {
        panic!("oh no");
    };

    let mut nums = vec![];

    for part_module in low {
        let mut modules = modules.clone();

        'outer: for i in 0.. {
            let mut signals = VecDeque::new();
            signals.push_back((Pulse::Low, "", "broadcaster"));
            while let Some((pulse, sender, module_name)) = signals.pop_front() {
                let Some(module) = modules.get_mut(module_name) else {
                    continue;
                };

                let Some(next_pulse) = module.module_type.on_recv(sender, pulse) else {
                    continue;
                };

                for conn in &module.connections {
                    signals.push_back((next_pulse, module_name, conn));
                }

                if &module_name == upper_name
                    && &sender == part_module
                    && matches!(pulse, Pulse::High)
                {
                    nums.push(i + 1);
                    break 'outer;
                }
            }
        }
    }

    lcm(&nums)
}

// https://github.com/TheAlgorithms/Rust/blob/7d2aa9e8be79cd23c36aa99cbfa66b520b132035/src/math/lcm_of_n_numbers.rs
fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
