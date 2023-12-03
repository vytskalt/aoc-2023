#[derive(Clone, Copy)]
enum Character {
    Digit(u32),
    Symbol,
    Nothing,
}

impl Character {
    fn parse(char: char) -> Self {
        if char == '.' {
            Character::Nothing
        } else if let Some(digit) = char.to_digit(10) {
            Character::Digit(digit)
        } else {
            Character::Symbol
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

    fn is_part_number(&self, x1: usize, x2: usize, y: usize) -> bool {
        fn check(board: &Board, x: isize, y: isize) -> bool {
            if x < 0 || y < 0 {
                return false;
            }

            matches!(board.get(x as usize, y as usize), Some(Character::Symbol))
        }

        if check(self, x1 as isize - 1, y as isize) || check(self, x2 as isize + 1, y as isize) {
            return true;
        }

        for x in x1 as isize - 1..=x2 as isize + 1 {
            if check(self, x, y as isize + 1) || check(self, x, y as isize - 1) {
                return true;
            }
        }

        false
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> u32 {
    let board = Board::parse(input);
    (0..board.chars.len())
        .map(|y| {
            find_numbers(&board, y)
                .filter(|(_, x1, x2)| board.is_part_number(*x1, *x2, y))
                .map(|(num, _, _)| num)
                .sum::<u32>()
        })
        .sum()
}

fn find_numbers(board: &Board, y: usize) -> impl Iterator<Item = (u32, usize, usize)> + '_ {
    let mut current_number: Option<(usize, u32)> = None;
    board.chars[y]
        .iter()
        .chain(std::iter::once(&Character::Nothing))
        .enumerate()
        .filter_map(move |(x, char)| {
            match char {
                Character::Digit(digit) => match &mut current_number {
                    Some((_, acc)) => {
                        *acc = *acc * 10 + digit;
                    }
                    None => {
                        current_number = Some((x, *digit));
                    }
                },
                _ => {
                    if let Some((x1, acc)) = current_number {
                        let x2 = x - 1;
                        current_number = None;
                        return Some((acc, x1, x2));
                    }
                }
            }

            None
        })
}
