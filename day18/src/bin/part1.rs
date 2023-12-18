use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn from_char(char: char) -> Self {
        match char {
            'U' => Self::Up,
            'D' => Self::Down,
            'R' => Self::Right,
            'L' => Self::Left,
            _ => panic!("Unknown direction char '{char}'"),
        }
    }

    fn dx(&self) -> isize {
        match self {
            Self::Up => 0,
            Self::Down => 0,
            Self::Right => 1,
            Self::Left => -1,
        }
    }

    fn dy(&self) -> isize {
        match self {
            Self::Up => -1,
            Self::Down => 1,
            Self::Right => 0,
            Self::Left => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    direction: Direction,
    length: u32,
}

impl Instruction {
    fn parse(line: &str) -> Self {
        let mut parts = line.split(' ');
        let direction = Direction::from_char(parts.next().unwrap().chars().nth(0).unwrap());
        let length = parts.next().unwrap().parse::<u32>().unwrap();
        Self { direction, length }
    }
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    tiles: Vec<bool>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        let tiles = vec![false; width * height];
        Self {
            width,
            height,
            tiles,
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<bool> {
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

    fn set(&mut self, x: isize, y: isize, filled: bool) {
        if x < 0 || y < 0 {
            return;
        }

        if x as usize >= self.width || y as usize >= self.height {
            return;
        }

        *self
            .tiles
            .get_mut(y as usize * self.width + x as usize)
            .unwrap() = filled;
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> usize {
    let mut points = vec![];

    let (mut curr_x, mut curr_y, mut max_x, mut max_y, mut min_x, mut min_y) = (0, 0, 0, 0, 0, 0);
    for ins in input.lines().map(Instruction::parse) {
        for _ in 0..ins.length {
            points.push((curr_x, curr_y));
            curr_x += ins.direction.dx();
            curr_y += ins.direction.dy();

            max_x = max_x.max(curr_x);
            max_y = max_y.max(curr_y);
            min_x = min_x.min(curr_x);
            min_y = min_y.min(curr_y);
        }
    }

    let mut grid = Grid::new((max_x - min_x + 1) as usize, (max_y - min_y + 1) as usize);
    for (x, y) in &points {
        grid.set(min_x.abs() + *x, min_y.abs() + *y, true);
    }

    let mut checked_points = HashSet::new();
    let mut result = points.len();

    for start_x in 0..grid.width as isize {
        for start_y in 0..grid.height as isize {
            if grid.get(start_x, start_y).unwrap() {
                continue;
            }

            if checked_points.contains(&(start_x, start_y)) {
                continue;
            }

            let mut queue = VecDeque::new();
            queue.push_back((start_x, start_y));
            let mut reachable_count = 0;
            let mut is_inside = true;

            while let Some((x, y)) = queue.pop_front() {
                for dir in [
                    Direction::Up,
                    Direction::Down,
                    Direction::Right,
                    Direction::Left,
                ] {
                    let Some(is_filled) = grid.get(x + dir.dx(), y + dir.dy()) else {
                        is_inside = false;
                        continue;
                    };

                    if !is_filled && checked_points.insert((x + dir.dx(), y + dir.dy())) {
                        queue.push_back((x + dir.dx(), y + dir.dy()));
                        reachable_count += 1;
                    }
                }
            }

            if is_inside {
                result += reachable_count;
            }
        }
    }

    result
}
