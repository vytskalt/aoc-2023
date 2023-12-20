use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
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

#[derive(Debug)]
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

fn process<'a>(input: &'a str) -> u32 {
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

    let (mut low, mut high) = (0, 0);
    for _ in 0..1000 {
        let mut signals = VecDeque::new();
        signals.push_back((Pulse::Low, "", "broadcaster"));
        while let Some((pulse, sender, module_name)) = signals.pop_front() {
            match pulse {
                Pulse::Low => low += 1,
                Pulse::High => high += 1,
            }

            let Some(module) = modules.get_mut(module_name) else {
                continue;
            };

            let Some(next_pulse) = module.module_type.on_recv(sender, pulse) else {
                continue;
            };

            for conn in &module.connections {
                signals.push_back((next_pulse, module_name, conn));
            }
        }
    }

    low * high
}
