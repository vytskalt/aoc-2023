use anyhow::Context;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(char: char) -> anyhow::Result<Direction> {
        match char {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => anyhow::bail!("Unknown direction char '{char}'"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct NodeId([char; 3]);

impl NodeId {
    fn parse(id: &str) -> anyhow::Result<Self> {
        let id = id
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| anyhow::anyhow!("Id not 3 characters"))?;

        Ok(NodeId(id))
    }

    fn is_starting(&self) -> bool {
        self.0[2] == 'A'
    }

    fn is_ending(&self) -> bool {
        self.0[2] == 'Z'
    }
}

#[derive(Debug)]
struct Node {
    left: NodeId,
    right: NodeId,
}

impl Node {
    fn parse(line: &str) -> anyhow::Result<(NodeId, Self)> {
        let id = NodeId::parse(&line[0..3])?;
        let left = NodeId::parse(&line[7..10])?;
        let right = NodeId::parse(&line[12..15])?;
        Ok((id, Self { left, right }))
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input).unwrap();
    println!("{}", output);
}

fn process(input: &str) -> anyhow::Result<usize> {
    let mut lines = input.lines();
    let directions = lines
        .next()
        .with_context(|| "Directions line not found")?
        .chars()
        .map(Direction::from_char)
        .collect::<Result<Vec<_>, _>>()?;

    let _ = lines.next();
    let nodes = lines
        .map(Node::parse)
        .collect::<Result<HashMap<_, _>, _>>()?;

    let lengths = nodes
        .iter()
        .filter(|(id, _)| id.is_starting())
        .map(|(_, node)| {
            let mut current_node = node;
            for (index, direction) in directions.iter().cycle().enumerate() {
                let next_node_id = match direction {
                    Direction::Left => current_node.left,
                    Direction::Right => current_node.right,
                };

                if next_node_id.is_ending() {
                    return Ok(index + 1);
                }

                current_node = nodes
                    .get(&next_node_id)
                    .with_context(|| "Did not find node {next_node_id}")?;
            }

            unreachable!()
        })
        .collect::<Result<Vec<_>, anyhow::Error>>()?;

    Ok(lcm(&lengths[..]))
}

// https://github.com/TheAlgorithms/Rust/blob/7d2aa9e8be79cd23c36aa99cbfa66b520b132035/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
