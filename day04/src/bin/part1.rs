use anyhow::Context;

#[derive(Debug)]
struct Card {
    winning: Vec<u32>,
    actual: Vec<u32>,
}

impl Card {
    fn parse(line: &str) -> anyhow::Result<Self> {
        let mut numbers = line
            .split(": ")
            .nth(1)
            .with_context(|| "No numbers provided")?
            .split(" | ");

        let winning = numbers
            .next()
            .with_context(|| "No winning numbers provided")?
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>())
            .collect::<Result<_, _>>()?;

        let actual = numbers
            .next()
            .with_context(|| "No actual numbers provided")?
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>())
            .collect::<Result<_, _>>()?;

        Ok(Self { winning, actual })
    }

    fn match_count(&self) -> u32 {
        self.winning
            .iter()
            .filter(|w| self.actual.iter().any(|a| a == *w))
            .count() as u32
    }

    fn score(&self) -> u32 {
        let matches = self.match_count(); 
        if matches > 0 {
            2_u32.pow(matches - 1)
        } else {
            0
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input).unwrap();
    println!("{}", output);
}

fn process(input: &str) -> anyhow::Result<u32> {
    input
        .lines()
        .map(Card::parse)
        .map(|card| card.map(|card| card.score()))
        .sum()
}
