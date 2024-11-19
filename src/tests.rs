// src/tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    use petgraph::algo::dijkstra;
    use petgraph::graph::UnGraph;

    #[test]
    fn test_dijkstra() {
        let mut graph = UnGraph::<(), ()>::new_undirected();
        let node1 = graph.add_node(());
        let node2 = graph.add_node(());
        let node3 = graph.add_node(());
        let node4 = graph.add_node(());

        graph.add_edge(node1, node2, ());
        graph.add_edge(node2, node3, ());
        graph.add_edge(node3, node4, ());

        let start_node_index = node1;
        let target_node_index = node4;

        let distances = dijkstra(&graph, start_node_index, Some(target_node_index), |_| 1);

        assert_eq!(distances.get(&target_node_index), Some(&3));
    }
}