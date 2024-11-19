// src/data_loader.rs
use std::fs::File;
use std::io::{BufRead, BufReader};
use petgraph::graph::UnGraph;

pub fn load_graph(file_path: &str) -> UnGraph<(), ()> {
    let file = File::open(file_path).expect("File not found");
    let reader = BufReader::new(file);

    let mut graph = UnGraph::<(), ()>::new_undirected();
    let mut node_map = std::collections::HashMap::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            let node1 = parts[0].parse::<usize>().expect("Invalid node ID");
            let node2 = parts[1].parse::<usize>().expect("Invalid node ID");

            let node1_index = *node_map.entry(node1).or_insert_with(|| graph.add_node(()));
            let node2_index = *node_map.entry(node2).or_insert_with(|| graph.add_node(()));

            graph.add_edge(node1_index, node2_index, ());
        }
    }

    graph
}