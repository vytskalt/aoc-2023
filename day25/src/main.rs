use std::collections::{HashMap, HashSet};

use petgraph::{
    dot::{Config, Dot},
    graph::UnGraph,
    visit::Dfs,
};

fn parse<'a>(line: &'a str) -> (&'a str, Vec<&'a str>) {
    let mut split = line.split(": ");
    let name = split.next().unwrap();
    let connections = split.next().unwrap().split(' ').collect::<Vec<_>>();
    (name, connections)
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    println!("{}", output);
}

fn process(input: &str) -> u32 {
    let components = input
        .lines()
        .map(|line| parse(line))
        .collect::<Vec<(&str, Vec<&str>)>>();

    let mut graph = UnGraph::<&str, ()>::default();
    let mut nodes = HashMap::new();

    for (name, conns) in &components {
        if !nodes.contains_key(name) {
            nodes.insert(name, graph.add_node(name));
        }

        for conn in conns {
            if !nodes.contains_key(conn) {
                nodes.insert(conn, graph.add_node(conn));
            }
        }
    }

    let mut edges = HashMap::new();
    for (name, connections) in &components {
        let node = nodes.get(name).unwrap();
        for conn in connections {
            let lol = nodes.get(conn).unwrap();
            let id = graph.add_edge(*node, *lol, ());
            edges.insert((*name, *conn), id);
            edges.insert((*conn, *name), id);
        }
    }

    let dot_string = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    std::fs::write("graph.dot", dot_string).unwrap();

    println!("Graph saved as graph.dot");
    println!("Visualize with: neato -T png -O graph.dot");
    println!("Enter connections separated by space (abc/def): ");

    let mut uinput = String::new();
    std::io::stdin()
        .read_line(&mut uinput)
        .expect("Failed to read line");

    for conn in uinput.split_whitespace() {
        let mut split = conn.split("/");
        let first = split.next().unwrap();
        let second = split.next().unwrap();
        let edge = edges.get(&(first, second)).unwrap();
        graph.remove_edge(*edge);
    }

    let mut visited = HashSet::<usize>::new();
    let mut result = 1;
    for node in nodes.values() {
        if visited.contains(&node.index()) {
            continue;
        }

        let mut dfs = Dfs::new(&graph, *node);
        let mut curr = 0;
        while let Some(nx) = dfs.next(&graph) {
            visited.insert(nx.index());
            curr += 1;
        }
        result *= curr;
    }

    result
}

