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

#[derive(Debug, Clone)]
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

    // this is surprisingly fast enough
    let mut result = 0;
    for idx in 0..grid.bricks.len() {
        let mut the = grid.clone();
        the.bricks.remove(idx);
        loop {
            if the.bricks.is_empty() {
                break;
            }

            let mut idxj = 0;
            let mut any_removed = false;
            loop {
                if !the.has_support(&the.bricks[idxj]) {
                    the.bricks.remove(idxj);
                    any_removed = true;
                    idxj += 1;
                    result += 1;
                }

                idxj += 1;
                if idxj >= the.bricks.len() {
                    break;
                }
            }

            if !any_removed {
                break;
            }
        }
    }
    result
}
