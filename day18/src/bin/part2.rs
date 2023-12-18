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
            '3' => Self::Up,
            '1' => Self::Down,
            '0' => Self::Right,
            '2' => Self::Left,
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
    length: u64,
}

impl Instruction {
    fn parse(line: &str) -> Self {
        let bruh = line
            .split(' ')
            .nth(2)
            .unwrap()
            .strip_prefix("(#")
            .unwrap()
            .strip_suffix(')')
            .unwrap();
        let length = u64::from_str_radix(&bruh[0..5], 16).unwrap();
        let direction = Direction::from_char(bruh.chars().nth(5).unwrap());
        Self { direction, length }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> i64 {
    let mut vertices = vec![];

    let mut perimeter = 0;
    let mut curr_x = 0;
    let mut curr_y = 0;
    for instruction in input.lines().map(Instruction::parse) {
        vertices.push((curr_x, curr_y));
        curr_x += instruction.length as i64 * instruction.direction.dx() as i64;
        curr_y += instruction.length as i64 * instruction.direction.dy() as i64;
        perimeter += instruction.length;
    }

    let n = vertices.len();
    let mut sum = 0;

    for i in 0..n {
        let j = (i + 1) % n;
        sum += (vertices[i].0 * vertices[j].1) - (vertices[j].0 * vertices[i].1);
    }

    sum.abs() / 2 + (perimeter as i64) / 2 + 1
}
