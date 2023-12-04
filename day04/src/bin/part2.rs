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

    fn match_count(&self) -> usize {
        self.winning
            .iter()
            .filter(|w| self.actual.iter().any(|a| a == *w))
            .count()
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input).unwrap();
    println!("{}", output);
}

fn process(input: &str) -> anyhow::Result<usize> {
    let mut counts = Vec::new();
    input
        .lines()
        .map(Card::parse)
        .enumerate()
        .map(|(i, card)| {
            card.map(|card| {
                let count = *counts.get(i).unwrap_or(&0) + 1;
                let matches = card.match_count();
                for j in 1..=matches {
                    let idx = i + j;
                    if idx >= counts.len() {
                        counts.resize(idx + 1, 0);
                    }
                    counts[idx] += count;
                }
                count
            })
        })
        .sum()
}
