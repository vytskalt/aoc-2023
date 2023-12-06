use anyhow::Context;

#[derive(Debug)]
struct Race {
    best_time: u64,
    distance: u64,
}

impl Race {
    fn num_of_ways_to_beat(&self) -> u64 {
        let mut n = 0;
        let mut was_found = false;
        for hold_steps in 1..self.distance {
            let time = self.distance / hold_steps;
            if hold_steps + time < self.best_time {
                was_found = true;
                n += 1;
            } else if was_found {
                break;
            }
        }
        n
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input).unwrap();
    println!("{}", output);
}

fn process(input: &str) -> anyhow::Result<u64> {
    let race = parse_races(input)?;
    Ok(race.num_of_ways_to_beat())
}

fn parse_races(input: &str) -> anyhow::Result<Race> {
    let mut lines = input.lines();
    let best_time = lines
        .next()
        .with_context(|| "Time line not found")?
        .strip_prefix("Time: ")
        .with_context(|| "Time prefix not found")?
        .chars()
        .filter_map(|c| c.to_digit(10))
        .fold(0u64, |acc, digit| acc * 10 + digit as u64);

    let distance = lines
        .next()
        .with_context(|| "Distance line not found")?
        .strip_prefix("Distance: ")
        .with_context(|| "Distance Prefix not found")?
        .chars()
        .filter_map(|c| c.to_digit(10))
        .fold(0u64, |acc, digit| acc * 10 + digit as u64);

    Ok(Race {
        best_time,
        distance,
    })
}
