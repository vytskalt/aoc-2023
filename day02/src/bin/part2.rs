use anyhow::Context;

#[derive(Debug, Default)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input).unwrap();
    println!("{}", output);
}

fn process(input: &str) -> anyhow::Result<u32> {
    let result = input
        .lines()
        .map(parse_game)
        .map(|sets| {
            sets.map(|sets| {
                sets.iter().fold(CubeSet::default(), |acc, set| {
                    use std::cmp::max;
                    CubeSet {
                        red: max(acc.red, set.red),
                        green: max(acc.green, set.green),
                        blue: max(acc.blue, set.blue),
                    }
                })
            })
        })
        .map(|set| set.map(|set| set.red * set.green * set.blue))
        .sum::<anyhow::Result<_>>()?;
    Ok(result)
}

fn parse_game(input: &str) -> anyhow::Result<Vec<CubeSet>> {
    let mut parts = input.split(':').skip(1);
    let sets = parts
        .next()
        .with_context(|| "Input has no colon")?
        .split(';')
        .map(parse_cube_set)
        .collect::<anyhow::Result<_>>()?;

    Ok(sets)
}

fn parse_cube_set(input: &str) -> anyhow::Result<CubeSet> {
    let mut set = CubeSet::default();
    input.split(',').try_fold(&mut set, |set, cube| {
        let mut parts = cube.split(' ').skip(1);
        let count = &parts
            .next()
            .with_context(|| "No cube count present")?
            .parse::<u32>()?;
        let color = parts.next().with_context(|| "No cube color present")?;
        match color {
            "red" => set.red += count,
            "green" => set.green += count,
            "blue" => set.blue += count,
            _ => return Err(anyhow::anyhow!("Unknown cube color {color}")),
        }
        Ok(set)
    })?;

    Ok(set)
}
