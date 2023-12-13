#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Ash,
    Rocks,
}

impl Tile {
    fn from_char(char: char) -> Self {
        match char {
            '.' => Self::Ash,
            '#' => Self::Rocks,
            _ => panic!("Invalid tile char '{char}'"),
        }
    }
}

#[derive(Debug)]
struct Pattern {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Pattern {
    fn parse(part: &str) -> Self {
        let tiles = part
            .lines()
            .flat_map(|line| line.chars().map(Tile::from_char))
            .collect::<Vec<_>>();
        let width = part.lines().next().unwrap().len();
        let height = tiles.len() / width;

        Self {
            width,
            height,
            tiles,
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<Tile> {
        if x < 0 || y < 0 {
            return None;
        }

        let (x, y) = (x as usize, y as usize);
        if x >= self.width || y >= self.height {
            return None;
        }

        self.tiles.get(y * self.width + x).copied()
    }

    fn row_matches(&self, y1: isize, y2: isize) -> Option<bool> {
        for x in 0..self.width as isize {
            let tile1 = self.get(x, y1)?;
            let tile2 = self.get(x, y2)?;
            if tile1 != tile2 {
                return Some(false);
            }
        }

        Some(true)
    }

    fn col_matches(&self, x1: isize, x2: isize) -> Option<bool> {
        for y in 0..self.height as isize {
            let tile1 = self.get(x1, y)?;
            let tile2 = self.get(x2, y)?;
            if tile1 != tile2 {
                return Some(false);
            }
        }

        Some(true)
    }

    fn calc_reflections(&self) -> u32 {
        'outer: for x in 0..self.width as isize {
            let matches = self.col_matches(x, x + 1);
            if !matches.unwrap_or(false) {
                continue;
            }

            for i in 1.. {
                let x1 = x - i;
                let x2 = x + i + 1;
                match self.col_matches(x1, x2) {
                    Some(true) => continue,
                    Some(false) => continue 'outer,
                    None => break,
                }
            }

            return x as u32 + 1;
        }

        'outer: for y in 0..self.height as isize {
            let matches = self.row_matches(y, y + 1);
            if !matches.unwrap_or(false) {
                continue;
            }

            for i in 1.. {
                let y1 = y - i;
                let y2 = y + i + 1;
                match self.row_matches(y1, y2) {
                    Some(true) => continue,
                    Some(false) => continue 'outer,
                    None => break,
                }
            }

            return (y as u32 + 1) * 100;
        }

        panic!("No reflections found");
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(Pattern::parse)
        .map(|pattern| pattern.calc_reflections())
        .sum::<u32>()
}
