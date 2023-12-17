use pathfinding::directed::dijkstra::dijkstra;

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

    fn right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<u32>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let tiles = input
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
            .collect::<Vec<_>>();
        let width = input.lines().next().unwrap().len();
        let height = tiles.len() / width;

        Self {
            width,
            height,
            tiles,
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<u32> {
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
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct State {
    x: isize,
    y: isize,
    direction: Direction,
    consecutive: u32,
}

impl State {
    fn next(&self, direction: Direction) -> Self {
        let consecutive = if self.direction == direction {
            self.consecutive + 1
        } else {
            0
        };

        Self {
            x: self.x + direction.dx(),
            y: self.y + direction.dy(),
            direction,
            consecutive,
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> u32 {
    let grid = Grid::parse(input);

    let start = State {
        x: 0,
        y: 0,
        consecutive: 0,
        direction: Direction::East,
    };

    let result = dijkstra(
        &start,
        |&state| {
            let available_directions = if state.consecutive < 2 {
                [
                    Some(state.direction),
                    Some(state.direction.right()),
                    Some(state.direction.left()),
                ]
            } else {
                [
                    None,
                    Some(state.direction.right()),
                    Some(state.direction.left()),
                ]
            };

            available_directions
                .iter()
                .filter_map(Option::as_ref)
                .map(move |dir| state.next(*dir))
                .filter_map(|s| {
                    let weight = grid.get(s.x, s.y)?;
                    Some((s, weight))
                })
                .collect::<Vec<_>>()
        },
        |&p| p.x == grid.width as isize - 1 && p.y == grid.height as isize - 1,
    );

    result.unwrap().1
}
