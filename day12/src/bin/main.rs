use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl Condition {
    fn from_char(char: char) -> Self {
        match char {
            '.' => Condition::Operational,
            '#' => Condition::Damaged,
            '?' => Condition::Unknown,
            _ => panic!("Unknown condition char '{char}'"),
        }
    }
}

#[derive(Debug)]
struct ConditionRecord {
    conditions: Vec<Condition>,
    damaged_groups: Vec<u32>,
}

impl ConditionRecord {
    fn parse(line: &str) -> Self {
        let mut parts = line.split(' ');
        let conditions = parts
            .next()
            .unwrap()
            .chars()
            .map(|c| Condition::from_char(c))
            .collect();

        let damaged_groups = parts
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        Self {
            conditions,
            damaged_groups,
        }
    }

    fn unfold(self) -> Self {
        let mut conditions = Vec::with_capacity(self.conditions.len() * 6 - 1);
        let mut damaged_groups = Vec::with_capacity(self.damaged_groups.len() * 5);

        for i in 0..5 {
            for condition in &self.conditions {
                conditions.push(*condition);
            }

            if i != 4 {
                conditions.push(Condition::Unknown);
            }

            for group in &self.damaged_groups {
                damaged_groups.push(*group);
            }
        }

        Self {
            conditions,
            damaged_groups,
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let (output1, output2) = process(input);
    println!("Part 1: {}", output1);
    println!("Part 2: {}", output2);
}

fn process(input: &str) -> (u64, u64) {
    let records = input.lines().map(ConditionRecord::parse);

    let part1 = records
        .clone()
        .map(|record| {
            find_possible_combinations(
                &mut HashMap::new(),
                &record.damaged_groups,
                &record.conditions,
                State::default(),
            )
        })
        .sum();

    let part2 = records
        .map(|record| record.unfold())
        .map(|record| {
            find_possible_combinations(
                &mut HashMap::new(),
                &record.damaged_groups,
                &record.conditions,
                State::default(),
            )
        })
        .sum();

    (part1, part2)
}

#[derive(Default, PartialEq, Eq, Hash)]
struct State {
    damaged_groups_index: usize,
    conditions_index: usize,
    consecutive_count: u32,
    last_damaged: bool,
}

fn find_possible_combinations(
    cache: &mut HashMap<State, u64>,
    damaged_groups: &[u32],
    conditions: &[Condition],
    state: State,
) -> u64 {
    if let Some(cached) = cache.get(&state) {
        return *cached;
    }

    let check_consecutive = || {
        *damaged_groups[state.damaged_groups_index..]
            .first()
            .unwrap_or(&(state.consecutive_count + 1))
            == state.consecutive_count
    };

    let operational = || {
        if state.last_damaged && !check_consecutive() {
            return None;
        }

        Some(State {
            damaged_groups_index: state.damaged_groups_index
                + if state.last_damaged { 1 } else { 0 },
            conditions_index: state.conditions_index + 1,
            consecutive_count: 0,
            last_damaged: false,
        })
    };

    let damaged = || {
        Some(State {
            damaged_groups_index: state.damaged_groups_index,
            conditions_index: state.conditions_index + 1,
            consecutive_count: state.consecutive_count + 1,
            last_damaged: true,
        })
    };

    let Some(condition) = conditions.get(state.conditions_index) else {
        if state.last_damaged
            && check_consecutive()
            && damaged_groups[state.damaged_groups_index..].len() == 1
        {
            return 1;
        } else if !state.last_damaged && damaged_groups[state.damaged_groups_index..].is_empty() {
            return 1;
        }

        return 0;
    };

    let result = match condition {
        Condition::Operational => operational()
            .map(|s| find_possible_combinations(cache, damaged_groups, conditions, s))
            .unwrap_or(0),
        Condition::Damaged => damaged()
            .map(|s| find_possible_combinations(cache, damaged_groups, conditions, s))
            .unwrap_or(0),
        Condition::Unknown => {
            let operational = operational()
                .map(|s| find_possible_combinations(cache, damaged_groups, conditions, s))
                .unwrap_or(0);

            let damaged = damaged()
                .map(|s| find_possible_combinations(cache, damaged_groups, conditions, s))
                .unwrap_or(0);

            operational + damaged
        }
    };
    cache.insert(state, result);
    result
}
