use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    fn parse(str: &str) -> Self {
        let mut split = str.split(", ").map(|x| x.trim().parse::<i64>().unwrap());
        let x = split.next().unwrap();
        let y = split.next().unwrap();
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    initial_pos: Vec2,
    velocity: Vec2,
}

impl Hailstone {
    fn parse(line: &str) -> Self {
        let mut split = line.split(" @ ").map(|x| Vec2::parse(x));
        let initial_pos = split.next().unwrap();
        let velocity = split.next().unwrap();
        Self {
            initial_pos,
            velocity,
        }
    }

    fn area_range(&self, min: i64, max: i64) -> RangeInclusive<i64> {
        let x_range = calc(self.velocity.x, self.initial_pos.x, min, max);
        let y_range = calc(self.velocity.y, self.initial_pos.y, min, max);

        let intersection = intersect(&x_range, &y_range);
        if *intersection.start() < 0 {
            0..=*intersection.end()
        } else {
            intersection
        }
    }
}

fn calc(vel: i64, initial: i64, min: i64, max: i64) -> RangeInclusive<i64> {
    let lower = (min - initial) / vel;
    let upper = (max - initial) / vel;
    if vel < 0 {
        upper..=lower
    } else {
        lower..=upper
    }
}

fn intersect(a: &RangeInclusive<i64>, b: &RangeInclusive<i64>) -> RangeInclusive<i64> {
    std::cmp::max(*a.start(), *b.start())..=std::cmp::min(*a.end(), *b.end())
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn lines_intersect(line1_start: Vec2, line1_end: Vec2, line2_start: Vec2, line2_end: Vec2) -> bool {
    let m1 = if line1_end.x as f64 - line1_start.x as f64 != 0.0 {
        (line1_end.y as f64 - line1_start.y as f64) / (line1_end.x as f64 - line1_start.x as f64)
    } else {
        f64::INFINITY
    };

    let m2 = if line2_end.x as f64 - line2_start.x as f64 != 0.0 {
        (line2_end.y as f64 - line2_start.y as f64) / (line2_end.x as f64 - line2_start.x as f64)
    } else {
        f64::INFINITY
    };

    if m1 == m2 {
        return false;
    }

    let x = (m1 * line1_start.x as f64 - m2 * line2_start.x as f64 + line2_start.y as f64
        - line1_start.y as f64)
        / (m1 - m2);
    let y = m1 * (x - line1_start.x as f64) + line1_start.y as f64;

    if (line1_start.x.min(line1_end.x) as f64 <= x
        && x <= line1_start.x.max(line1_end.x) as f64
        && line1_start.y.min(line1_end.y) as f64 <= y
        && y <= (line1_start.y.max(line1_end.y)) as f64)
        && (line2_start.x.min(line2_end.x) as f64 <= x
            && x <= (line2_start.x.max(line2_end.x) as f64)
            && line2_start.y.min(line2_end.y) as f64 <= y
            && y <= (line2_start.y.max(line2_end.y)) as f64)
    {
        true
    } else {
        false
    }
}

fn process(input: &str) -> u32 {
    const MIN: i64 = 200000000000000;
    const MAX: i64 = 400000000000000;

    let ranges = input.lines().map(Hailstone::parse).collect::<Vec<_>>();

    let mut result = 0;
    for i in 0..ranges.len() {
        let irange = ranges[i].area_range(MIN, MAX);

        let ix_lower = ranges[i].initial_pos.x + irange.start() * ranges[i].velocity.x;
        let ix_upper = ranges[i].initial_pos.x + irange.end() * ranges[i].velocity.x;

        let iy_lower = ranges[i].initial_pos.y + irange.start() * ranges[i].velocity.y;
        let iy_upper = ranges[i].initial_pos.y + irange.end() * ranges[i].velocity.y;

        let line1_start = Vec2 {
            x: ix_lower,
            y: iy_lower,
        };
        let line1_end = Vec2 {
            x: ix_upper,
            y: iy_upper,
        };

        for j in i + 1..ranges.len() {
            let jrange = ranges[j].area_range(MIN, MAX);

            let jx_lower = ranges[j].initial_pos.x + jrange.start() * ranges[j].velocity.x;
            let jx_upper = ranges[j].initial_pos.x + jrange.end() * ranges[j].velocity.x;

            let jy_lower = ranges[j].initial_pos.y + jrange.start() * ranges[j].velocity.y;
            let jy_upper = ranges[j].initial_pos.y + jrange.end() * ranges[j].velocity.y;

            let line2_start = Vec2 {
                x: jx_lower,
                y: jy_lower,
            };
            let line2_end = Vec2 {
                x: jx_upper,
                y: jy_upper,
            };

            if lines_intersect(line1_start, line1_end, line2_start, line2_end) {
                result += 1;
            }
        }
    }
    result
}
