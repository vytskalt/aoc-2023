use std::{collections::HashSet, mem};

#[derive(Debug, Clone, Copy)]
enum MirrorType {
    Forward,
    Backward,
}

impl MirrorType {
    fn next_direction(&self, current: Direction) -> Direction {
        match self {
            Self::Forward => match current {
                Direction::North => Direction::East,
                Direction::East => Direction::North,
                Direction::South => Direction::West,
                Direction::West => Direction::South,
            },
            Self::Backward => match current {
                Direction::North => Direction::West,
                Direction::East => Direction::South,
                Direction::South => Direction::East,
                Direction::West => Direction::North,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum SplitterType {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Mirror(MirrorType),
    Splitter(SplitterType),
}

impl Tile {
    fn from_char(char: char) -> Self {
        match char {
            '.' => Self::Empty,
            '/' => Self::Mirror(MirrorType::Forward),
            '\\' => Self::Mirror(MirrorType::Backward),
            '|' => Self::Splitter(SplitterType::Vertical),
            '-' => Self::Splitter(SplitterType::Horizontal),
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
            .get(y as usize * self.height + x as usize)
            .copied()
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Beam {
    x: isize,
    y: isize,
    direction: Direction,
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn simulate(grid: &Grid, x: isize, y: isize, direction: Direction) -> usize {
    let mut states = HashSet::new();
    let mut energized = HashSet::new();
    let mut current_beams: Vec<Beam> = vec![Beam { x, y, direction }];
    let mut next_beams: Vec<Beam> = vec![];

    loop {
        if current_beams.is_empty() {
            break;
        }

        for mut beam in current_beams.drain(..) {
            let Some(tile) = grid.get(beam.x, beam.y) else {
                continue;
            };

            energized.insert((beam.x, beam.y));

            if !states.insert(beam) {
                continue;
            }

            match tile {
                Tile::Empty => {}
                Tile::Splitter(t) => match (beam.direction, t) {
                    (Direction::North | Direction::South, SplitterType::Horizontal) => {
                        next_beams.push(Beam {
                            x: beam.x + 1,
                            y: beam.y,
                            direction: Direction::East,
                        });
                        next_beams.push(Beam {
                            x: beam.x - 1,
                            y: beam.y,
                            direction: Direction::West,
                        });
                        continue;
                    }
                    (Direction::West | Direction::East, SplitterType::Vertical) => {
                        next_beams.push(Beam {
                            x: beam.x,
                            y: beam.y + 1,
                            direction: Direction::South,
                        });
                        next_beams.push(Beam {
                            x: beam.x,
                            y: beam.y - 1,
                            direction: Direction::North,
                        });
                        continue;
                    }
                    _ => {}
                },
                Tile::Mirror(t) => {
                    beam.direction = t.next_direction(beam.direction);
                }
            }

            beam.x += beam.direction.dx();
            beam.y += beam.direction.dy();
            next_beams.push(beam);
        }

        mem::swap(&mut current_beams, &mut next_beams);
    }

    energized.len()
}

fn process(input: &str) -> usize {
    let grid = Grid::parse(input);
    let mut most: usize = 0;

    for x in 0..grid.width as isize {
        most = most.max(simulate(&grid, x, 0, Direction::South));
        most = most.max(simulate(
            &grid,
            x,
            grid.height as isize - 1,
            Direction::North,
        ));
    }

    for y in 0..grid.height as isize {
        most = most.max(simulate(&grid, 0, y, Direction::East));
        most = most.max(simulate(&grid, grid.width as isize - 1, y, Direction::West));
    }

    most
}
