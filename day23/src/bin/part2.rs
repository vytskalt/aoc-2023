use std::collections::{HashMap, HashSet, VecDeque};
use bit_set::BitSet;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Path,
    Forest,
}

impl Tile {
    fn from_char(char: char) -> Self {
        match char {
            '.' | '^' | '>' | 'v' | '<' => Self::Path,
            '#' => Self::Forest,
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

        self.tiles.get(self.index(x, y)).copied()
    }

    fn index(&self, x: isize, y: isize) -> usize {
        y as usize * self.width + x as usize
    }

    fn start(&self) -> (isize, isize) {
        (1, 0)
    }

    fn end(&self) -> (isize, isize) {
        (self.width as isize - 2, self.height as isize - 1)
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Edge {
    target: (isize, isize),
    distance: u32,
}

#[derive(Debug, Default)]
struct Graph {
    nodes: HashSet<(isize, isize)>,
    edges: HashMap<(isize, isize), HashSet<Edge>>,
}

impl Graph {
    fn add_node(&mut self, pos: (isize, isize)) -> bool {
        self.nodes.insert(pos)
    }

    fn add_edge(&mut self, source: (isize, isize), target: (isize, isize), distance: u32) {
        self.edges
            .entry(source)
            .or_default()
            .insert(Edge { target, distance });
        self.edges.entry(target).or_default().insert(Edge {
            target: source,
            distance,
        });
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> u32 {
    let grid = Grid::parse(input);
    let mut graph = Graph::default();

    let mut queue = VecDeque::new();
    graph.add_node((1, 0));
    queue.push_back(((1, 1), (1, 0)));

    while let Some(((x, y), (last_x, last_y))) = queue.pop_front() {
        let point = find_point(&grid, (last_x, last_y), (x, y));
        if graph.add_node(point.end) {
            for dir in point.available_directions {
                queue.push_back(((point.end.0 + dir.dx(), point.end.1 + dir.dy()), point.end));
            }
        }
        graph.add_edge((last_x, last_y), point.end, point.length);
    }

    find_longest_path(&grid, &graph, &mut BitSet::new(), grid.start()).unwrap()
}

#[derive(Debug)]
struct Point {
    end: (isize, isize),
    length: u32,
    available_directions: Vec<Direction>,
}

fn find_point(grid: &Grid, mut last_pos: (isize, isize), start_pos: (isize, isize)) -> Point {
    let mut pos = start_pos;
    let mut len = 0;
    loop {
        let mut directions = vec![];
        for dir in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            let (nx, ny) = (pos.0 + dir.dx(), pos.1 + dir.dy());
            if (nx, ny) == last_pos {
                continue;
            }

            let Some(Tile::Path) = grid.get(nx, ny) else {
                continue;
            };

            directions.push(dir);
        }

        if directions.len() != 1 {
            return Point {
                end: pos,
                length: len,
                available_directions: directions,
            };
        }

        let dir = directions.first().unwrap();
        last_pos = pos;
        pos.0 += dir.dx();
        pos.1 += dir.dy();
        len += 1;
    }
}

fn find_longest_path(grid: &Grid, graph: &Graph, visited: &mut BitSet, pos: (isize, isize)) -> Option<u32> {
    if pos == grid.end() {
        return Some(0);
    }

    visited.insert(grid.index(pos.0, pos.1));

    let mut steps: i32 = -1;
    for edge in graph.edges.get(&pos).unwrap() {
        if visited.contains(grid.index(edge.target.0, edge.target.1)) {
            continue;
        }

        let mut new_visited: BitSet = visited.clone();
        if let Some(s) = find_longest_path(grid, graph, &mut new_visited, edge.target) {
            steps = steps.max(1 + s as i32 + edge.distance as i32);
        }
    }

    if steps == -1 {
        None
    } else {
        Some(steps as u32)
    }
}
