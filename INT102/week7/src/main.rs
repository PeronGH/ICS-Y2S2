use petgraph::prelude::UnGraph;

fn main() {
    let mut graph = UnGraph::<usize, f64>::new_undirected();

    let a = graph.add_node(0);
    let b = graph.add_node(1);
    let c = graph.add_node(2);
    let d = graph.add_node(3);
    let e = graph.add_node(4);

    graph.add_edge(a, b, 1.0);
    graph.add_edge(a, c, 4.0);
    graph.add_edge(b, c, 2.0);
    graph.add_edge(b, d, 5.0);
    graph.add_edge(c, d, 1.0);
    graph.add_edge(c, e, 8.0);
    graph.add_edge(d, e, 3.0);

    let source = a;
    let bellman_ford_distances = week7::bellman_ford(&graph, source).unwrap();

    println!(
        "Shortest path distances from node {:?} are: {:?}",
        source, bellman_ford_distances
    );

    let floyd_warshall_distances = week7::floyd_warshall(&graph);

    println!(
        "Shortest path distances are: {:?}",
        floyd_warshall_distances
    );
}
