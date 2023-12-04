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
            .split(' ')
            .filter_map(|num| num.parse::<u32>().ok())
            .collect();

        let actual = numbers
            .next()
            .with_context(|| "No actual numbers provided")?
            .split(' ')
            .filter_map(|num| num.parse::<u32>().ok())
            .collect();

        Ok(Self { winning, actual })
    }

    fn score(&self) -> u32 {
        let mut score = 0;
        for num in &self.winning {
            if !self.actual.iter().any(|x| x == num) {
                continue;
            }

            if score == 0 {
                score = 1;
            } else {
                score *= 2;
            }
        }
        score
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
