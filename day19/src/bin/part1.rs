use std::collections::HashMap;

#[derive(Debug)]
struct MachinePart {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl MachinePart {
    fn parse(str: &str) -> Self {
        let (x, m, a, s) = str
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .map(|x| {
                let category = Category::from_char(x.chars().nth(0).unwrap());
                let num = x[2..].parse::<u32>().unwrap();
                (category, num)
            })
            .fold(
                (0, 0, 0, 0),
                |(x, m, a, s), (category, num)| match category {
                    Category::X => (num, m, a, s),
                    Category::M => (x, num, a, s),
                    Category::A => (x, m, num, s),
                    Category::S => (x, m, a, num),
                },
            );

        Self { x, m, a, s }
    }

    fn get(&self, category: Category) -> u32 {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
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

    fn does_match(&self, part: &MachinePart) -> bool {
        let num = part.get(self.category);
        match self.condition {
            RuleCondition::Lower => num < self.num,
            RuleCondition::Higher => num > self.num,
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

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> u32 {
    let mut parts = input.split("\n\n");
    let workflows = parts
        .next()
        .unwrap()
        .lines()
        .map(Workflow::parse)
        .map(|wf| (wf.name, wf))
        .collect::<HashMap<_, _>>();

    let wf_in = workflows.get("in").unwrap();

    parts
        .next()
        .unwrap()
        .lines()
        .map(MachinePart::parse)
        .filter(|part| {
            let mut curr = wf_in;
            loop {
                let mut action = &curr.default;

                for rule in &curr.rules {
                    if rule.does_match(&part) {
                        action = &rule.action;
                        break;
                    }
                }

                match action {
                    Action::Accept => return true,
                    Action::Reject => return false,
                    Action::Redirect(name) => {
                        curr = workflows.get(name).unwrap();
                    }
                }
            }
        })
        .map(|part| part.x + part.m + part.a + part.s)
        .sum::<u32>()
}
