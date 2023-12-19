use std::{
    collections::HashMap,
    ops::RangeInclusive,
};

#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
    default: Action<'a>,
}

impl<'a> Workflow<'a> {
    fn parse(line: &'a str) -> Self {
        let mut parts = line.split('{');
        let name = parts.next().unwrap();
        let stuff = parts
            .next()
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .collect::<Vec<_>>();

        let mut iter = stuff.into_iter().rev();
        let default = Action::parse(iter.next().unwrap());
        let rules = iter.map(Rule::parse).rev().collect::<Vec<_>>();

        Self {
            name,
            rules,
            default,
        }
    }
}

#[derive(Debug)]
enum RuleCondition {
    Lower,
    Higher,
}

impl RuleCondition {
    fn from_char(char: char) -> Self {
        match char {
            '<' => Self::Lower,
            '>' => Self::Higher,
            _ => panic!("Unknown rule cnodition '{char}'"),
        }
    }
}

#[derive(Debug)]
enum Action<'a> {
    Redirect(&'a str),
    Accept,
    Reject,
}

impl<'a> Action<'a> {
    fn parse(str: &'a str) -> Action<'a> {
        match str {
            "A" => Action::Accept,
            "R" => Action::Reject,
            _ => Action::Redirect(str),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct MachinePart {
    x: RangeInclusive<u32>,
    m: RangeInclusive<u32>,
    a: RangeInclusive<u32>,
    s: RangeInclusive<u32>,
}

impl MachinePart {
    fn get(&mut self, category: Category) -> &mut RangeInclusive<u32> {
        match category {
            Category::X => &mut self.x,
            Category::M => &mut self.m,
            Category::A => &mut self.a,
            Category::S => &mut self.s,
        }
    }

    fn count(&mut self) -> u64 {
        let ranges = [&self.x, &self.m, &self.a, &self.s];
        ranges
            .iter()
            .map(|range| (*range).clone().count() as u64)
            .fold(1, |acc, x| acc * x)
    }
}

#[derive(Debug, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn from_char(char: char) -> Self {
        match char {
            'x' => Category::X,
            'm' => Category::M,
            'a' => Category::A,
            's' => Category::S,
            _ => panic!("Unknown category char '{char}'"),
        }
    }
}

#[derive(Debug)]
struct Rule<'a> {
    category: Category,
    num: u32,
    condition: RuleCondition,
    action: Action<'a>,
}

impl<'a> Rule<'a> {
    fn parse(str: &'a str) -> Self {
        let mut parts = str.split(':');

        let first = parts.next().unwrap();
        let mut chars = first.chars();
        let category = Category::from_char(chars.next().unwrap());
        let condition = RuleCondition::from_char(chars.next().unwrap());
        let num = first[2..].parse::<u32>().unwrap();
        let action = Action::parse(parts.next().unwrap());

        Self {
            category,
            num,
            condition,
            action,
        }
    }

    fn check(&self, part: &mut MachinePart) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
        let range = part.get(self.category);
        match self.condition {
            RuleCondition::Lower => (*range.start()..=self.num - 1, self.num..=*range.end()),
            RuleCondition::Higher => (self.num + 1..=*range.end(), *range.start()..=self.num),
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn find_possible_parts(
    workflows: &HashMap<&str, Workflow>,
    curr_wf: &Workflow,
    mut part: MachinePart,
) -> u64 {
    let mut result = 0;

    for rule in &curr_wf.rules {
        let (new_range, other_range) = rule.check(&mut part);

        let mut next_part = part.clone();
        *next_part.get(rule.category) = new_range.clone();

        *part.get(rule.category) = other_range;
        if new_range.is_empty() {
            continue;
        }

        match rule.action {
            Action::Accept => {
                result += next_part.count();
                continue;
            }
            Action::Reject => continue,
            Action::Redirect(name) => {
                let next_workflow = workflows.get(name).unwrap();
                result += find_possible_parts(workflows, next_workflow, next_part);
            }
        }
    }

    match curr_wf.default {
        Action::Accept => result += part.count(),
        Action::Reject => {}
        Action::Redirect(name) => {
            let next_workflow = workflows.get(name).unwrap();
            result += find_possible_parts(workflows, next_workflow, part);
        }
    }

    result
}

fn process(input: &str) -> u64 {
    let workflows = input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(Workflow::parse)
        .map(|wf| (wf.name, wf))
        .collect::<HashMap<_, _>>();

    let wf_in = workflows.get("in").unwrap();
    let part = MachinePart {
        x: 1..=4000,
        m: 1..=4000,
        a: 1..=4000,
        s: 1..=4000,
    };
    find_possible_parts(&workflows, wf_in, part)
}
