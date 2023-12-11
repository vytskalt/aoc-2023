use anyhow::Context;
use std::collections::{HashSet, VecDeque};

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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn dx(&self) -> isize {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 0,
            Direction::West => -1,
        }
    }

    fn dy(&self) -> isize {
        match self {
            Direction::North => -1,
            Direction::East => 0,
            Direction::South => 1,
            Direction::West => 0,
        }
    }

    fn delta(&self) -> (isize, isize) {
        (self.dx(), self.dy())
    }

    fn reverse(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Connection(Direction, Direction);

impl Connection {
    fn next(&self, last_dir: Direction) -> anyhow::Result<Direction> {
        if last_dir == self.0 {
            Ok(self.1)
        } else if last_dir == self.1 {
            Ok(self.0)
        } else {
            anyhow::bail!("Invalid last direction {last_dir:?}");
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
            .map(|line| line.chars().map(|tile| Tile::from_char(tile)).collect())
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

    fn set(&mut self, x: isize, y: isize, tile: Tile) {
        if x >= 0 && y >= 0 {
            let Some(lines) = self.tiles.get_mut(y as usize) else {
                return;
            };

            let Some(tile_ref) = lines.get_mut(x as usize) else {
                return;
            };

            *tile_ref = tile;
        }
    }

    fn relative(&self, dx: isize, dy: isize, x: isize, y: isize) -> Option<Tile> {
        self.get(x + dx, y + dy)
    }

    fn scale(self, loop_tiles: &[(isize, isize)]) -> anyhow::Result<Self> {
        let mut new_grid = Grid {
            width: self.width * 2,
            height: self.height * 2,
            tiles: vec![vec![Tile::Ground; self.width * 2]; self.height * 2],
        };

        for (loop_x, loop_y) in loop_tiles {
            let Some(Tile::Pipe(conn)) = self.get(*loop_x, *loop_y) else {
                anyhow::bail!("Invalid loop tile coords {loop_x} {loop_y}");
            };

            let x = loop_x * 2;
            let y = loop_y * 2;
            new_grid.set(x, y, Tile::Pipe(conn));
            new_grid.set(
                x + conn.0.dx(),
                y + conn.0.dy(),
                Tile::Pipe(Connection(conn.0, conn.0.reverse())),
            );

            new_grid.set(
                x + conn.1.dx(),
                y + conn.1.dy(),
                Tile::Pipe(Connection(conn.1, conn.1.reverse())),
            );
        }

        Ok(new_grid)
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input).unwrap();
    println!("{}", output);
}

fn process(input: &str) -> anyhow::Result<u32> {
    let mut grid = Grid::parse(input)?;
    let mut other_grid = Grid::parse(input)?;
    let (starting_x, starting_y) = grid.starting_pos()?;

    let loop_dir = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ]
    .iter()
    .find(|dir| {
        let (dx, dy) = dir.delta();
        let Some(Tile::Pipe(conn)) = grid.relative(dx, dy, starting_x, starting_y) else {
            return false;
        };

        let (dx1, dy1) = conn.0.delta();
        let (dx2, dy2) = conn.1.delta();
        (-dx == dx1 && -dy == dy1) || (-dx == dx2 && -dy == dy2)
    })
    .with_context(|| "No pipe connecting to the starting position found")?;

    let mut last_dir = *loop_dir;
    let (mut current_x, mut current_y) = (starting_x + last_dir.dx(), starting_y + last_dir.dy());
    let mut loop_tiles = vec![(starting_x, starting_y)];

    loop {
        loop_tiles.push((current_x, current_y));
        let tile = grid.get(current_x, current_y);
        match tile {
            Some(Tile::Pipe(conn)) => {
                let dir = conn.next(last_dir.reverse())?;
                current_x += dir.dx();
                current_y += dir.dy();
                last_dir = dir;
            }
            Some(Tile::Start) => break,
            _ => anyhow::bail!("Encountered invalid tile: {tile:?}"),
        }
    }

    let start_tile = Tile::Pipe(Connection(*loop_dir, last_dir.reverse()));
    grid.set(starting_x, starting_y, start_tile);
    other_grid.set(starting_x, starting_y, start_tile);

    let grid = grid.scale(&loop_tiles)?;
    let mut inside = HashSet::new();
    let mut outside = HashSet::new();

    for start_x in 0..grid.width as isize {
        for start_y in 0..grid.height as isize {
            if !matches!(grid.get(start_x, start_y), Some(Tile::Ground)) {
                continue;
            }

            if inside.contains(&(start_x, start_y)) || outside.contains(&(start_x, start_y)) {
                continue;
            }

            let mut area = HashSet::new();
            let mut queue = VecDeque::new();
            let mut can_escape = false;

            queue.push_front((start_x, start_y));
            area.insert((start_x, start_y));

            while !queue.is_empty() {
                let (x, y) = queue.pop_front().unwrap();
                outside.insert((x, y));

                for dir in [
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ] {
                    let (dx, dy) = dir.delta();
                    let tile = grid.get(x + dx, y + dy);
                    match tile {
                        Some(Tile::Ground) => {
                            if !area.contains(&(x + dx, y + dy)) {
                                area.insert((x + dx, y + dy));
                                queue.push_back((x + dx, y + dy));
                            }
                        }
                        None => can_escape = true,
                        _ => {}
                    }
                }
            }

            if can_escape {
                outside.extend(area);
            } else {
                inside.extend(area);
            }
        }
    }

    let mut count = 0;
    for x in 0..grid.width as isize {
        for y in 0..grid.height as isize {
            let mut is_big_enough = true;
            let deltas = [(0, 0), (1, 0), (0, 1), (1, 1)];

            for (dx, dy) in deltas {
                if !inside.contains(&(dx + x, dy + y)) {
                    is_big_enough = false;
                    break;
                }
            }

            if !is_big_enough {
                continue;
            }

            count += 1;
            for (dx, dy) in deltas {
                inside.remove(&(dx + x, dy + y));
            }
        }
    }

    Ok(count)
}
