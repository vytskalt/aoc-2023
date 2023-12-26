use bit_set::BitSet;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

impl Tile {
    fn from_char(char: char) -> Self {
        match char {
            '.' => Self::Path,
            '#' => Self::Forest,
            '^' => Self::Slope(Direction::North),
            '>' => Self::Slope(Direction::East),
            'v' => Self::Slope(Direction::South),
            '<' => Self::Slope(Direction::West),
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

        self.tiles.get(self.index(x, y)).copied()
    }

    fn index(&self, x: isize, y: isize) -> usize {
        y as usize * self.width + x as usize
    }

    fn start(&self) -> (isize, isize) {
        (1, 0)
    }

    fn end(&self) -> (isize, isize) {
        (self.width as isize - 2, self.height as isize - 1)
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> i32 {
    let grid = Grid::parse(input);
    let mut visited = BitSet::with_capacity(grid.width * grid.height);
    let result = dfs(&grid, &mut visited, grid.start())
        .map(|x| x as i32)
        .unwrap_or(-1);
    result
}

fn dfs(grid: &Grid, visited: &mut BitSet, pos: (isize, isize)) -> Option<u32> {
    if pos == grid.end() {
        return Some(0);
    }

    visited.insert(grid.index(pos.0, pos.1));

    let mut steps: i32 = -1;
    for dir in [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ] {
        let (mut nx, mut ny) = (pos.0 + dir.dx(), pos.1 + dir.dy());

        let Some(tile) = grid.get(nx, ny) else {
            continue;
        };

        if visited.contains(grid.index(nx, ny)) {
            continue;
        }

        let mut new_visited: BitSet;
        let mut extra = 1;
        match tile {
            Tile::Path => {
                new_visited = visited.clone();
            }
            Tile::Forest => continue,
            Tile::Slope(slope_dir) => {
                new_visited = visited.clone();
                new_visited.insert(grid.index(nx, ny));
                nx += slope_dir.dx();
                ny += slope_dir.dy();
                extra += 1;
                if visited.contains(grid.index(nx, ny)) {
                    continue;
                }
            }
        }

        if let Some(s) = dfs(grid, &mut new_visited, (nx, ny)) {
            steps = steps.max(extra + s as i32);
        }
    }

    if steps == -1 {
        None
    } else {
        Some(steps as u32)
    }
}
