use petgraph::graph::UnGraph;
use week9::*;

fn main() {
    let mut graph = UnGraph::<&str, f64>::new_undirected();
    let nodes = vec![
        graph.add_node("A"),
        graph.add_node("B"),
        graph.add_node("C"),
        graph.add_node("D"),
    ];

    graph.extend_with_edges([
        (nodes[0], nodes[1], 10.0),
        (nodes[0], nodes[2], 15.0),
        (nodes[0], nodes[3], 20.0),
        (nodes[1], nodes[2], 35.0),
        (nodes[1], nodes[3], 25.0),
        (nodes[2], nodes[3], 30.0),
    ]);

    let (best_tour, best_cost) = travelling_salesman(&graph, nodes[0]);

    println!("Best tour: {:?}", best_tour);
    println!("Best cost: {:?}", best_cost);
}
