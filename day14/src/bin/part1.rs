#[derive(Debug, Clone, Copy)]
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

#[derive(Debug)]
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

    fn slide(&mut self) {
        for y in 0..self.height {
            for x in 0..self.height {
                let Some(Tile::RoundedRock) = self.get(x, y) else {
                    continue;
                };
                self.set(x, y, Tile::Empty);

                for i in 1..=y + 1 {
                    let below = self.get(x, y - i);
                    if !matches!(below, Some(Tile::Empty)) {
                        self.set(x, y - i + 1, Tile::RoundedRock);
                        break;
                    }
                }
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
    platform.slide();
    platform.load()
}
