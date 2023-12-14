use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Tile {
    RoundedRock,
    CubeRock,
    Empty,
}

impl Tile {
    fn from_char(char: char) -> Self {
        match char {
            'O' => Self::RoundedRock,
            '#' => Self::CubeRock,
            '.' => Self::Empty,
            _ => panic!("Unknown tile '{char}'"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Platform {
    width: isize,
    height: isize,
    tiles: Vec<Tile>,
}

impl Platform {
    fn parse(input: &str) -> Self {
        let tiles = input
            .lines()
            .flat_map(|line| line.chars().map(Tile::from_char))
            .collect::<Vec<_>>();
        let width = input.lines().next().unwrap().len() as isize;
        let height = tiles.len() as isize / width;

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

        if x >= self.width || y >= self.height {
            return None;
        }

        self.tiles
            .get(y as usize * self.height as usize + x as usize)
            .copied()
    }

    fn set(&mut self, x: isize, y: isize, tile: Tile) {
        if x < 0 || y < 0 {
            return;
        }

        if x >= self.width || y >= self.height {
            return;
        }

        *self
            .tiles
            .get_mut(y as usize * self.height as usize + x as usize)
            .unwrap() = tile;
    }

    fn cycle(&mut self) {
        self.slide(Direction::North);
        self.slide(Direction::West);
        self.slide(Direction::South);
        self.slide(Direction::East);
    }

    fn slide(&mut self, direction: Direction) {
        match direction {
            Direction::North => {
                for y in 0..self.height {
                    for x in 0..self.width {
                        self.check_coords(direction, x, y);
                    }
                }
            }
            Direction::East => {
                for x in (0..self.width).rev() {
                    for y in (0..self.height).rev() {
                        self.check_coords(direction, x, y);
                    }
                }
            }
            Direction::South => {
                for y in (0..self.height).rev() {
                    for x in (0..self.width).rev() {
                        self.check_coords(direction, x, y);
                    }
                }
            }
            Direction::West => {
                for x in 0..self.width {
                    for y in 0..self.height {
                        self.check_coords(direction, x, y);
                    }
                }
            }
        }
    }

    fn check_coords(&mut self, direction: Direction, x: isize, y: isize) {
        let Some(Tile::RoundedRock) = self.get(x, y) else {
            return;
        };
        self.set(x, y, Tile::Empty);

        let (mut dx, mut dy) = (0, 0);
        loop {
            dx += direction.dx();
            dy += direction.dy();

            let below = self.get(x + dx, y + dy);
            if !matches!(below, Some(Tile::Empty)) {
                self.set(
                    x + dx + (-direction.dx()),
                    y + dy + (-direction.dy()),
                    Tile::RoundedRock,
                );
                break;
            }
        }
    }

    fn load(&self) -> u32 {
        let mut load = 0;
        for y in 0..self.height {
            for x in 0..self.height {
                let Some(Tile::RoundedRock) = self.get(x, y) else {
                    continue;
                };

                load += self.height - y;
            }
        }
        load as u32
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> u32 {
    let mut platform = Platform::parse(input);
    let mut platforms = HashMap::new();

    const CYCLES: u64 = 1_000_000_000;
    let mut left = 0u64;

    loop {
        platform.cycle();
        left += 1;

        let entry = platforms.entry(platform.clone());
        let first_insert = entry.or_insert(left);
        if *first_insert != left {
            let distance = left - *first_insert;
            left = CYCLES - ((CYCLES - left) % distance);
        }

        if left == CYCLES {
            return platform.load();
        }
    }
}
