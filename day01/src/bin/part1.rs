use anyhow::Context;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input).unwrap();
    println!("{}", output);
}

fn process(input: &str) -> anyhow::Result<u32> {
    input
        .lines()
        .map(|line| {
            let (first, last) =
                extract_first_last(line).with_context(|| "Line does not have any digits")?;
            Ok(10 * first + last)
        })
        .sum()
}

fn extract_first_last(line: &str) -> Option<(u32, u32)> {
    let digits = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    Some((*digits.first()?, *digits.last()?))
}
