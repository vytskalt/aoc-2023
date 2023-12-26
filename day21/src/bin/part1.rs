use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Plot,
    Rock,
    Start,
}

impl Tile {
    fn from_char(char: char) -> Self {
        match char {
            '.' => Self::Plot,
            '#' => Self::Rock,
            'S' => Self::Start,
            _ => panic!("Unknown tile char '{char}'"),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn dx(&self) -> isize {
        match self {
            Self::North => 0,
            Self::East => 1,
            Self::South => 0,
            Self::West => -1,
        }
    }

    fn dy(&self) -> isize {
        match self {
            Self::North => -1,
            Self::East => 0,
            Self::South => 1,
            Self::West => 0,
        }
    }
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let tiles = input
            .lines()
            .flat_map(|line| line.chars().map(Tile::from_char))
            .collect::<Vec<_>>();
        let width = input.lines().next().unwrap().len();
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

        if x as usize >= self.width || y as usize >= self.height {
            return None;
        }

        self.tiles
            .get(y as usize * self.width + x as usize)
            .copied()
    }

    fn find_start(&self) -> (isize, isize) {
        for x in 0..self.width as isize {
            for y in 0..self.height as isize {
                if matches!(self.get(x, y), Some(Tile::Start)) {
                    return (x, y);
                }
            }
        }
        panic!("Starting pos not found");
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> usize {
    let grid = Grid::parse(input);
    let start = grid.find_start();

    let mut curr = HashSet::new();
    let mut next = HashSet::new();

    curr.insert(start);

    for _ in 0..64 {
        for (x, y) in curr.drain() {
            for dir in [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ] {
                let Some(tile) = grid.get(x + dir.dx(), y + dir.dy()) else {
                    continue;
                };

                if matches!(tile, Tile::Rock) {
                    continue;
                }

                next.insert((x + dir.dx(), y + dir.dy()));
            }
        }

        std::mem::swap(&mut curr, &mut next);
    }

    curr.len()
}

