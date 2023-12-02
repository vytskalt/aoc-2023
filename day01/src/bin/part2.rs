fn main() {
    let input = include_str!("./input.txt");
    let result = process(input);
    println!("{}", result);
}

fn process(input: &str) -> u32 {
    input.lines().map(|line| {
        let (first, last) = extract_first_last(line).unwrap();
        10 * first + last
    }).sum::<u32>()
}

fn extract_first_last(line: &str) -> Option<(u32, u32)> {
    let mut digits = vec![];

    for (i, c) in line.chars().enumerate() {
        if let Some(n) = c.to_digit(10) {
            digits.push(n);
            continue;
        }

        if let Some(n) = extract_from_end(&line[..i + 1]) {
            digits.push(n);
        }
    }

    Some((*digits.first()?, *digits.last()?))
}

fn extract_from_end(input: &str) -> Option<u32> {
    let digits = [
        ("nine", 9),
        ("eight", 8),
        ("seven", 7),
        ("six", 6),
        ("five", 5),
        ("four", 4),
        ("three", 3),
        ("two", 2),
        ("one", 1)
    ];

    for (digit, value) in digits {
        if input.ends_with(digit) {
            return Some(value);
        }
    }

    None
}
