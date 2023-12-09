fn main() {
    let input = include_str!("./input.txt");
    let output = process(input).unwrap();
    println!("{}", output);
}

fn process(input: &str) -> anyhow::Result<i32> {
    let sum = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>())
                .collect::<Result<_, _>>()
        })
        .map(|line| line.map(|line| process_line(line)))
        .map(|nums| {
            nums.map(|nums| {
                nums.iter()
                    .rev()
                    .map(|x| x.last().unwrap())
                    .fold(0, |acc, x| acc + x)
            })
        })
        .sum::<Result<i32, _>>()?;
    Ok(sum)
}

fn process_line(line: Vec<i32>) -> Vec<Vec<i32>> {
    let mut out = vec![line];
    loop {
        let last = out.last().unwrap();
        let gaming = last
            .iter()
            .zip(last.iter().skip(1))
            .map(|(a, b)| *b - *a)
            .collect::<Vec<_>>();

        let should_end = gaming.iter().all(|a| *a == 0);
        out.push(gaming);
        if should_end {
            break out;
        }
    }
}

