// src/main.rs
mod data_loader;
#[cfg(test)]
mod tests;
mod visualization;

use petgraph::prelude::*;
use petgraph::algo::dijkstra;

fn main() {
    let file_path = "data/facebook_combined.txt";
    let graph = data_loader::load_graph(file_path);

    let start_node = 0;
    let target_node = 107;

    let start_node_index = NodeIndex::new(start_node);
    let target_node_index = NodeIndex::new(target_node);

    let distances = dijkstra(&graph, start_node_index, Some(target_node_index), |_| 1);

    if let Some(distance) = distances.get(&target_node_index) {
        println!("The shortest path length between {} and {} is {}", start_node, target_node, distance);

        let path = petgraph::algo::dijkstra::dijkstra(&graph, start_node_index, Some(target_node_index), |_| 1)
            .keys()
            .cloned()
            .collect::<Vec<NodeIndex>>();

        visualization::visualize_graph(&graph, &path);
    } else {
        println!("No path found between {} and {}", start_node, target_node);
    }
}