use std::collections::HashSet;

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    galaxies: HashSet<(usize, usize)>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let width = input.lines().next().expect("Line to not be empty").len();
        let height = input.lines().count();

        let galaxies = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, char)| *char == '#')
                    .map(move |(x, _)| (x, y))
            })
            .collect::<HashSet<_>>();

        Self {
            width,
            height,
            galaxies,
        }
    }

    fn scale(self) -> Self {
        let mut empty_columns = vec![];
        for x in 0..self.width {
            let mut is_empty = true;
            for y in 0..self.height {
                if self.galaxies.contains(&(x, y)) {
                    is_empty = false;
                    break;
                }
            }
            if is_empty {
                empty_columns.push(x);
            }
        }

        let mut empty_rows = vec![];
        for y in 0..self.height {
            let mut is_empty = true;
            for x in 0..self.width {
                if self.galaxies.contains(&(x, y)) {
                    is_empty = false;
                    break;
                }
            }
            if is_empty {
                empty_rows.push(y);
            }
        }

        let mut new_galaxies = HashSet::new();
        for (x, y) in self.galaxies {
            let mut extra_x = 0;
            let mut extra_y = 0;

            for column in &empty_columns {
                if x > *column {
                    extra_x += 1;
                } else {
                    break;
                }
            }

            for row in &empty_rows {
                if y > *row {
                    extra_y += 1;
                } else {
                    break;
                }
            }

            new_galaxies.insert((x + extra_x, y + extra_y));
        }

        Self {
            width: self.width + empty_columns.len(),
            height: self.height + empty_rows.len(),
            galaxies: new_galaxies,
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> isize {
    let map = Map::parse(input);
    let scaled = map.scale();

    let mut sum = 0;
    for (x1, y1) in &scaled.galaxies {
        for (x2, y2) in &scaled.galaxies {
            if x1 == x2 && y1 == y2 {
                continue;
            }

            sum += (*x1 as isize - *x2 as isize).abs() + (*y1 as isize - *y2 as isize).abs();
        }
    }
    sum / 2
}
