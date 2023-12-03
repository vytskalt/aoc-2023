use arrayvec::ArrayVec;

#[derive(Clone, Copy)]
enum Character {
    Digit(u32),
    Gear,
    Nothing,
}

impl Character {
    fn parse(char: char) -> Self {
        if char == '*' {
            Character::Gear
        } else if let Some(digit) = char.to_digit(10) {
            Character::Digit(digit)
        } else {
            Character::Nothing
        }
    }
}

struct Board {
    chars: Vec<Vec<Character>>,
}

impl Board {
    fn parse(input: &str) -> Self {
        let chars = input
            .lines()
            .map(|line| line.chars().map(Character::parse).collect())
            .collect();
        Self { chars }
    }

    fn get(&self, x: usize, y: usize) -> Option<Character> {
        Some(*self.chars.get(y)?.get(x)?)
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> u32 {
    let board = Board::parse(input);
    board
        .chars
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, char)| matches!(char, Character::Gear))
                .filter_map(|(x, _)| find_gear_numbers(&board, x, y))
                .map(|(num1, num2)| num1 * num2)
                .sum::<u32>()
        })
        .sum()
}

fn find_gear_numbers(board: &Board, x: usize, y: usize) -> Option<(u32, u32)> {
    let dx = [-1, 0, 1, -1, 1, -1, 0, 1];
    let dy = [1, 1, 1, 0, 0, -1, -1, -1];
    let mut numbers = ArrayVec::<(usize, usize), 2>::new();

    for i in 0..8 {
        let rx = x as isize + dx[i];
        let ry = y as isize + dy[i];
        if rx < 0 || ry < 0 {
            continue;
        }

        let Some(Character::Digit(_)) = board.get(rx as usize, ry as usize) else {
            continue;
        };

        let start = find_number_start(board, rx as usize, ry as usize);
        if numbers
            .iter()
            .any(|comp| *comp == (start, ry as usize))
        {
            continue;
        }

        if numbers.is_full() {
            return None;
        }

        numbers.push((start, ry as usize));
    }

    if !numbers.is_full() {
        return None;
    }

    let (x1, y1) = numbers[0];
    let (x2, y2) = numbers[1];
    Some((find_number(board, x1, y1), find_number(board, x2, y2)))
}

fn find_number_start(board: &Board, x: usize, y: usize) -> usize {
    let mut start = x;
    while start > 0 && matches!(board.get(start - 1, y), Some(Character::Digit(_))) {
        start -= 1;
    }
    start
}

fn find_number(board: &Board, x: usize, y: usize) -> u32 {
    let mut acc = 0;
    for i in 0.. {
        let Some(Character::Digit(digit)) = board.get(x + i, y) else {
            break;
        };
        acc = acc * 10 + digit;
    }
    acc
}
