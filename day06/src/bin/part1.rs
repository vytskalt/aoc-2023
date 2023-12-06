use anyhow::Context;

#[derive(Debug)]
struct Race {
    best_time: u32,
    distance: u32,
}

impl Race {
    fn num_of_ways_to_beat(&self) -> u32 {
        (1..self.distance)
            .filter(|&hold_steps| {
                let time = self.distance / hold_steps;
                hold_steps + time < self.best_time
            })
            .count() as u32
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input).unwrap();
    println!("{}", output);
}

fn process(input: &str) -> anyhow::Result<u32> {
    let races = parse_races(input)?;
    let output = races
        .map(|race| race.map(|race| race.num_of_ways_to_beat()))
        .try_fold(1, |acc, num| num.map(|num| acc * num))?;
    Ok(output)
}

fn parse_races(input: &str) -> anyhow::Result<impl Iterator<Item = anyhow::Result<Race>> + '_> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .with_context(|| "Time line not found")?
        .strip_prefix("Time: ")
        .with_context(|| "Time prefix not found")?
        .split_whitespace()
        .map(|num| num.parse::<u32>());

    let distances = lines
        .next()
        .with_context(|| "Distance line not found")?
        .strip_prefix("Distance: ")
        .with_context(|| "Distance Prefix not found")?
        .split_whitespace()
        .map(|num| num.parse::<u32>());

    Ok(times.zip(distances).map(|(best_time, distance)| {
        Ok(Race {
            best_time: best_time?,
            distance: distance?,
        })
    }))
}
