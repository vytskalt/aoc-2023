use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Brick {
    start: Vec3,
    end: Vec3,
}

impl Brick {
    fn parse(line: &str) -> Self {
        let mut split = line.split("~");
        let start = Vec3::parse(split.next().unwrap());
        let end = Vec3::parse(split.next().unwrap());
        assert!(start.x <= end.x);
        assert!(start.y <= end.y);
        assert!(start.z <= end.z);
        Self { start, end }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Vec3 {
    x: u32,
    y: u32,
    z: u32,
}

impl Vec3 {
    fn parse(str: &str) -> Self {
        let mut split = str.split(",");
        let x = split.next().unwrap().parse::<u32>().unwrap();
        let y = split.next().unwrap().parse::<u32>().unwrap();
        let z = split.next().unwrap().parse::<u32>().unwrap();
        Self { x, y, z }
    }
}

#[derive(Debug)]
struct Grid {
    bricks: Vec<Brick>,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let bricks = input.lines().map(Brick::parse).collect::<Vec<_>>();
        Self { bricks }
    }

    fn has_support(&self, brick: &Brick) -> bool {
        let z = brick.start.z - 1;
        if z == 0 {
            return true;
        }

        for other_brick in &self.bricks {
            if other_brick.end.z == z && intersect(brick, other_brick) {
                return true;
            }
        }

        false
    }

    fn check_unsafe(&self, brick: &Brick) -> Option<Brick> {
        let z = brick.start.z - 1;
        if z == 0 {
            return None;
        }

        let mut supports = Vec::with_capacity(2);
        for other_brick in &self.bricks {
            if other_brick.end.z == z && intersect(brick, other_brick) {
                supports.push(other_brick);
                if supports.len() >= 2 {
                    return None;
                }
            }
        }

        supports.first().cloned().cloned()
    }
}

fn intersect(brick1: &Brick, brick2: &Brick) -> bool {
    let (x1, y1, x2, y2) = (brick1.start.x, brick1.start.y, brick1.end.x, brick1.end.y);
    let (x3, y3, x4, y4) = (brick2.start.x, brick2.start.y, brick2.end.x, brick2.end.y);
    !(x2 < x3 || x4 < x1 || y2 < y3 || y4 < y1)
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> usize {
    let mut grid = Grid::parse(input);
    grid.bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z));
    for idx in 0..grid.bricks.len() {
        while !grid.has_support(&grid.bricks[idx]) {
            let brick = &mut grid.bricks[idx];
            brick.start.z -= 1;
            brick.end.z -= 1;
        }
    }

    let mut unsafe_bricks = HashSet::new();
    for brick in &grid.bricks {
        let Some(unsafe_brick) = grid.check_unsafe(&brick) else {
            continue;
        };

        unsafe_bricks.insert(unsafe_brick);
    }

    grid.bricks.len() - unsafe_bricks.len()
}
