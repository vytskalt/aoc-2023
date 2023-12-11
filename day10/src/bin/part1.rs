use anyhow::Context;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Start,
    Ground,
    Pipe(Connection),
}

impl Tile {
    fn from_char(char: char) -> anyhow::Result<Self> {
        match char {
            'S' => Ok(Self::Start),
            '.' => Ok(Self::Ground),
            '|' => Ok(Self::Pipe(Connection(Direction::North, Direction::South))),
            '-' => Ok(Self::Pipe(Connection(Direction::East, Direction::West))),
            'L' => Ok(Self::Pipe(Connection(Direction::North, Direction::East))),
            'J' => Ok(Self::Pipe(Connection(Direction::North, Direction::West))),
            '7' => Ok(Self::Pipe(Connection(Direction::South, Direction::West))),
            'F' => Ok(Self::Pipe(Connection(Direction::South, Direction::East))),
            _ => Err(anyhow::anyhow!("Unknown tile character '{char}'")),
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
    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Connection(Direction, Direction);

impl Connection {
    fn next(&self, last_dx: isize, last_dy: isize) -> anyhow::Result<(isize, isize)> {
        let (dx1, dy1) = self.0.delta();
        let (dx2, dy2) = self.1.delta();

        if last_dx == dx1 && last_dy == dy1 {
            Ok((dx2, dy2))
        } else if last_dx == dx2 && last_dy == dy2 {
            Ok((dx1, dy1))
        } else {
            anyhow::bail!("Invalid last dx and dy ({last_dx} {last_dy})");
        }
    }
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
}

impl Grid {
    fn parse(input: &str) -> anyhow::Result<Self> {
        let tiles = input
            .lines()
            .map(|line| line.chars().map(Tile::from_char).collect())
            .collect::<Result<Vec<Vec<_>>, _>>()?;

        let height = tiles.len();
        let width = tiles
            .first()
            .with_context(|| "Grid must not be empty")?
            .len();

        Ok(Grid {
            width,
            height,
            tiles,
        })
    }

    fn starting_pos(&self) -> anyhow::Result<(isize, isize)> {
        for x in 0..self.width as isize {
            for y in 0..self.height as isize {
                if matches!(self.get(x, y), Some(Tile::Start)) {
                    return Ok((x, y));
                }
            }
        }

        anyhow::bail!("Starting position not found");
    }

    fn get(&self, x: isize, y: isize) -> Option<Tile> {
        if x >= 0 && y >= 0 {
            self.tiles.get(y as usize)?.get(x as usize).copied()
        } else {
            None
        }
    }

    fn relative(&self, dx: isize, dy: isize, x: isize, y: isize) -> Option<Tile> {
        self.get(x + dx, y + dy)
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input).unwrap();
    println!("{}", output);
}

fn process(input: &str) -> anyhow::Result<u32> {
    let grid = Grid::parse(input)?;
    let (starting_x, starting_y) = grid.starting_pos()?;

    let (loop_start_dx, loop_start_dy) = [(0, 1), (0, -1), (-1, 0), (1, 0)]
        .iter()
        .find(|(dx, dy)| {
            let Some(Tile::Pipe(conn)) = grid.relative(*dx, *dy, starting_x, starting_y) else {
                return false;
            };

            let (dx1, dy1) = conn.0.delta();
            let (dx2, dy2) = conn.1.delta();
            (-dx == dx1 && -dy == dy1) || (-dx == dx2 && -dy == dy2)
        })
        .with_context(|| "No pipe connecting to the starting position found")?;

    let (mut last_dx, mut last_dy) = (*loop_start_dx, *loop_start_dy);
    let (mut current_x, mut current_y) = (starting_x + loop_start_dx, starting_y + loop_start_dy);
    let mut count = 0;

    loop {
        let tile = grid.get(current_x, current_y);
        match tile {
            Some(Tile::Pipe(conn)) => {
                let (dx, dy) = conn.next(-last_dx, -last_dy)?;
                current_x = current_x + dx;
                current_y = current_y + dy;
                last_dx = dx;
                last_dy = dy;
                count += 1;
            }
            Some(Tile::Start) => break,
            _ => anyhow::bail!("Encountered invalid tile: {tile:?}"),
        }
    }

    Ok((count + 1) / 2)
}
