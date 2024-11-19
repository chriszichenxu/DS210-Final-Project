// src/visualization.rs
use petgraph::prelude::*;
use plotters::prelude::*;

pub fn visualize_graph(graph: &UnGraph<(), ()>, path: &[NodeIndex]) {
    // Simple force-directed layout
    let mut positions = vec![(0.0, 0.0); graph.node_count()];
    let k = 1.0 / (graph.node_count() as f64).sqrt();
    let iterations = 1000;

    for _ in 0..iterations {
        for i in 0..graph.node_count() {
            let mut force = (0.0, 0.0);
            for j in 0..graph.node_count() {
                if i != j {
                    let dx = positions[j].0 - positions[i].0;
                    let dy = positions[j].1 - positions[i].1;
                    let d = ((dx * dx + dy * dy) as f64).sqrt();
                    let repulsion = k * k / d;
                    let attraction = d * d / k;
                    force.0 += (repulsion - attraction) * dx / d;
                    force.1 += (repulsion - attraction) * dy / d;
                }
            }
            positions[i].0 += force.0 / 10.0;
            positions[i].1 += force.1 / 10.0;
        }
    }

    let root = BitMapBackend::new("output.png", (600, 400)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Facebook Network", ("sans-serif", 40))
        .build_cartesian_2d(0..600, 0..400)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    for (i, pos) in positions.iter().enumerate() {
        let x = pos.0 as i32 + 300;
        let y = pos.1 as i32 + 200;

        if path.contains(&NodeIndex::new(i)) {
            let _ = chart.draw_series(std::iter::once(Circle::new((x, y), 5, &RED.mix(0.5))));
        } else {
            let _ = chart.draw_series(std::iter::once(Circle::new((x, y), 3, &BLACK.mix(0.5))));
        }
    }

    for edge in graph.edge_indices() {
        let source = graph.edge_endpoints(edge).unwrap().0;
        let target = graph.edge_endpoints(edge).unwrap().1;

        let source_pos = positions[source.index()];
        let target_pos = positions[target.index()];

        let x1 = source_pos.0 as i32 + 300;
        let y1 = source_pos.1 as i32 + 200;
        let x2 = target_pos.0 as i32 + 300;
        let y2 = target_pos.1 as i32 + 200;

        let _ = chart.draw_series(std::iter::once(PathElement::new(vec![(x1, y1), (x2, y2)], &BLACK.mix(0.5))));
    }
}