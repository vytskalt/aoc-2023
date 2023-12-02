use anyhow::Context;

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

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
    let result = input.lines().map(parse_game).try_fold(0, |acc, game| {
        let game = game?;
        let sets = game.sets;
        let is_invalid = sets
            .iter()
            .any(|set| set.red > 12 || set.green > 13 || set.blue > 14);
        Ok::<_, anyhow::Error>(if is_invalid { acc } else { acc + game.id })
    })?;
    Ok(result)
}

fn parse_game(input: &str) -> anyhow::Result<Game> {
    let mut parts = input.split(':');
    let id = parts
        .next()
        .with_context(|| "Game input is empty")?
        .strip_prefix("Game ")
        .with_context(|| "No game prefix present")?
        .parse::<u32>()?;

    let sets = parts
        .next()
        .with_context(|| "Input has no colon")?
        .split(';')
        .map(parse_cube_set)
        .collect::<anyhow::Result<_>>()?;

    Ok(Game { id, sets })
}

fn parse_cube_set(input: &str) -> anyhow::Result<CubeSet> {
    let mut set = CubeSet::default();
    input.split(',').try_fold(&mut set, |acc, cube| {
        let mut parts = cube.split(' ').skip(1);
        let count = &parts
            .next()
            .with_context(|| "No cube count present")?
            .parse::<u32>()?;
        let color = parts.next().with_context(|| "No cube color present")?;
        match color {
            "red" => acc.red += count,
            "green" => acc.green += count,
            "blue" => acc.blue += count,
            _ => return Err(anyhow::anyhow!("Unknown cube color {color}")),
        }
        Ok(acc)
    })?;

    Ok(set)
}
