use petgraph::prelude::DiGraph;

pub fn q1() {
    let mut graph = DiGraph::<f64, f64>::new();

    let a = graph.add_node(0.);
    let b = graph.add_node(f64::INFINITY);
    let c = graph.add_node(f64::INFINITY);
    let d = graph.add_node(f64::INFINITY);
    let e = graph.add_node(f64::INFINITY);

    graph.add_edge(b, a, 1.);
    graph.add_edge(a, c, 5.);
    graph.add_edge(a, d, 2.);
    graph.add_edge(c, d, 2.);
    graph.add_edge(d, e, 2.);
    graph.add_edge(c, e, 1.);
    graph.add_edge(b, d, -2.);
    graph.add_edge(c, b, -3.);

    let result = week7::bellman_ford(&graph, a).unwrap();
    println!("Q1: {:?}", result);
}
