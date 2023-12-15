use smallvec::{smallvec, SmallVec};

const SIZE: usize = 256;

#[derive(Debug, Eq, PartialEq)]
struct Label<'a>(&'a str);

impl Label<'_> {
    fn hash(&self) -> usize {
        self.0
            .chars()
            .fold(0, |acc, c| (acc + c as usize) * 17 % SIZE)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Operation {
    Add(u8),
    Remove,
}

fn parse<'a>(part: &'a str) -> (Label<'a>, Operation) {
    if part.chars().nth(part.len() - 1).unwrap() == '-' {
        return (Label(&part[0..part.len() - 1]), Operation::Remove);
    }

    let mut parts = part.split('=');
    let label = Label(parts.next().unwrap());
    let index = parts.next().unwrap().parse::<u8>().unwrap();
    (label, Operation::Add(index))
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn process<'a>(input: &'a str) -> usize {
    let mut slots: [SmallVec<[(&'a str, u8); 5]>; SIZE] = std::array::from_fn(|_| smallvec![]);

    'outer: for part in input.strip_suffix('\n').unwrap().split(',') {
        let (label, op) = parse(part);
        let lengths = slots.get_mut(label.hash()).unwrap();

        match op {
            Operation::Add(length) => {
                for (ilabel, ilength) in lengths.iter_mut() {
                    if label.0 == *ilabel {
                        *ilength = length;
                        continue 'outer;
                    }
                }

                lengths.push((label.0, length));
            }
            Operation::Remove => {
                let Some(index) = lengths.iter().position(|(ilabel, _)| label.0 == *ilabel) else {
                    continue;
                };
                lengths.remove(index);
            }
        }
    }

    slots
        .iter()
        .enumerate()
        .flat_map(|(box_number, lengths)| {
            lengths
                .iter()
                .enumerate()
                .map(move |(slot, (_, length))| (box_number + 1) * (slot + 1) * *length as usize)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let (label, op) = parse("rn=69");
        assert_eq!(label, Label("rn"));
        assert_eq!(op, Operation::Add(69));

        let (label, op) = parse("abcde-");
        assert_eq!(label, Label("abcde"));
        assert_eq!(op, Operation::Remove);
    }
}
