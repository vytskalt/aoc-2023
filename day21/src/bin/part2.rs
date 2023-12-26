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
        let is_outside = x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize;

        let x = (x % self.width as isize + self.width as isize) % self.width as isize;
        let y = (y % self.height as isize + self.height as isize) % self.height as isize;

        let tile = self
            .tiles
            .get(y as usize * self.width + x as usize)
            .copied();

        match tile {
            Some(Tile::Start) if is_outside => Some(Tile::Plot),
            _ => tile,
        }
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

    fn fill_unreachable_spots(&mut self) {
        // Fill spots like this:
        // .#.
        // #.#
        // .#.
        // There may be bigger spots like this but they aren't in my input so not gonna check
        // for them
        for x in 0..self.width as isize {
            'outer: for y in 0..self.height as isize {
                let Some(Tile::Plot) = self.get(x, y) else {
                    continue;
                };

                for dir in [
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ] {
                    if !matches!(self.get(x + dir.dx(), y + dir.dy()), Some(Tile::Rock)) {
                        continue 'outer;
                    }
                }

                *self
                    .tiles
                    .get_mut(y as usize * self.width + x as usize)
                    .unwrap() = Tile::Rock;
            }
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> u128 {
    let mut grid = Grid::parse(input);
    grid.fill_unreachable_spots();
    let (start_x, start_y) = grid.find_start();

    let steps = 26501365 - 1;

    let mut result: u128 = 0;
    for i in 0..=steps + 1 {
        let lower_x = start_x + -1 * steps - 1 + i;
        let upper_x = start_x + steps + 1 - i;

        let length = (upper_x - lower_x).abs();

        let mut tile_count_up = 0;
        let mut tile_count_down = 0;
        for x in (lower_x..lower_x + grid.width as isize * 2).step_by(2) {
            if !matches!(grid.get(x, start_y + i).unwrap(), Tile::Rock) {
                tile_count_up += 1;
            }

            if i != 0 && !matches!(grid.get(x, start_y - i).unwrap(), Tile::Rock) {
                tile_count_down += 1;
            }
        }

        let what = length / (grid.width * 2) as isize;
        result += what as u128 * tile_count_up as u128;
        result += what as u128 * tile_count_down as u128;
        let start = lower_x + what * (grid.width as isize * 2);

        for x in (start..=upper_x).step_by(2) {
            if !matches!(grid.get(x, start_y + i).unwrap(), Tile::Rock) {
                result += 1;
            }

            if i != 0 && !matches!(grid.get(x, start_y - i).unwrap(), Tile::Rock) {
                result += 1;
            }
        }
    }
    result
}
