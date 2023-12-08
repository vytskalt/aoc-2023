use anyhow::Context;
use std::collections::HashMap;

#[derive(Debug)]
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
    const STARTING: NodeId = NodeId(['A', 'A', 'A']);
    const ENDING: NodeId = NodeId(['Z', 'Z', 'Z']);

    fn parse(id: &str) -> anyhow::Result<Self> {
        let id = id
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| anyhow::anyhow!("Id not 3 characters"))?;

        Ok(NodeId(id))
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

fn process(input: &str) -> anyhow::Result<u32> {
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

    let mut current_node = nodes
        .get(&NodeId::STARTING)
        .with_context(|| "Starting node not found")?;

    for (index, direction) in directions.iter().cycle().enumerate() {
        let next_node_id = match direction {
            Direction::Left => current_node.left,
            Direction::Right => current_node.right,
        };

        if next_node_id == NodeId::ENDING {
            return Ok(index as u32 + 1);
        }

        current_node = nodes
            .get(&next_node_id)
            .with_context(|| "Did not find node {next_node_id}")?;
    }

    unreachable!()
}
