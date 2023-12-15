fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn hash(input: &str) -> u32 {
    input.chars().fold(0, |acc, c| {
        (acc + c as u32) * 17 % 256
    })
}

fn process(input: &str) -> u32 {
    input.split(',').map(hash).sum()
}
