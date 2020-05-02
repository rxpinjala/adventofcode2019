fn main() {
    let edges = orbits::read_from_input().unwrap();
    println!("Part 1: {}", orbits::count_indirect_edges(&edges));

    println!("Part 2: {}", orbits::shortest_path_length(&edges, "YOU", "SAN").unwrap());
}

mod orbits
{

use std::collections::BTreeSet;
use std::env;
use std::fs;

#[derive(Debug)]
pub struct Edge {
    src: String,
    dst: String
}

impl Edge {
    pub fn new(src: &str, dst: &str) -> Self {
        Edge { src: src.to_string(), dst: dst.to_string() }
    }
}

pub fn parse_edge(s: &str) -> Result<Edge, &'static str> {
    let parts: Vec<&str> = s.split(")").collect();
    if parts.len() == 2 {
        Ok(Edge::new(parts[0], parts[1]))
    } else {
        Err("Bad input line")
    }
}

pub fn read_from_input() -> Result<Vec<Edge>, &'static str> {
    let input_path: &String = &env::args().nth(1).unwrap();
    let input_data = fs::read_to_string(input_path).map_err(|_| "error reading input file")?;
    read_from_string(&input_data)
}

pub fn read_from_string(s: &str) -> Result<Vec<Edge>, &'static str> {
    s.lines().map(parse_edge).collect()
}

fn count_node_edges(edges: &[Edge], node: &str) -> u32 {
    let mut stack: Vec<&str> = Vec::new();
    let mut count: u32 = 0;

    for edge in edges.iter() {
        if edge.src == node {
            count = count + 1;
            stack.push(&edge.dst);
        }
    }

    while stack.len() > 0 {
        let e: &str = stack.pop().unwrap();
        for edge in edges.iter() {
            if edge.src == e {
                count = count + 1;
                stack.push(&edge.dst);
            }
        }
    }

    count
}

pub fn count_indirect_edges(edges: &[Edge]) -> u32 {
    let src_nodes: BTreeSet<String> = edges.iter().map(|e| e.src.clone()).collect();
    src_nodes.iter().map(|n| count_node_edges(edges, n)).fold(0, |sum, c| sum + c)
}

#[test]
fn provided_count_test() {
    let input = read_from_string(
"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L").unwrap();

        assert_eq!(count_indirect_edges(&input[..]), 42);
}

pub fn nodes_reachable_from(edges: &[Edge], start_nodes: BTreeSet<String>) -> BTreeSet<String> {
    let mut result: BTreeSet<String> = BTreeSet::new();
    for edge in edges.iter() {
        if start_nodes.contains(&edge.src) && !result.contains(&edge.dst){
            result.insert(edge.dst.to_string());
        }

        if start_nodes.contains(&edge.dst) && !result.contains(&edge.src){
            result.insert(edge.src.to_string());
        }
    }

    result
}

pub fn shortest_path_length(edges: &[Edge], start: &str, end: &str) -> Option<u32> {
    let mut count = 0;
    let mut current_nodes: BTreeSet<String> = BTreeSet::new();
    current_nodes.insert(start.to_string());

    while !current_nodes.contains(end) {
        count = count + 1;
        current_nodes = nodes_reachable_from(edges, current_nodes);

        if current_nodes.len() == 0 {
            return None;
        }

        if count > 1000 {
            return None;
        }
    }

    Some(count - 2)
}

#[test]
fn test_shortest_path() {
    let edges = read_from_string(
"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN").unwrap();
    assert_eq!(shortest_path_length(&edges, "YOU", "SAN"), Some(4));
}

}