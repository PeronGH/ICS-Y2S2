use petgraph::graph::{NodeIndex, UnGraph};
use std::fmt::Debug;

pub fn travelling_salesman<N: Clone + Debug>(
    graph: &UnGraph<N, f64>,
    start_node: NodeIndex,
) -> (Vec<NodeIndex>, f64) {
    let path = vec![start_node];
    let current_cost = 0.0;
    let mut best_tour = Vec::new();
    let mut best_cost = f64::INFINITY;

    tsp_helper(
        graph,
        start_node,
        path,
        current_cost,
        &mut best_tour,
        &mut best_cost,
    );

    (best_tour, best_cost)
}

fn lower_bound<N: Clone + Debug>(graph: &UnGraph<N, f64>, path: &Vec<NodeIndex>) -> f64 {
    let mut lb = 0.0;
    for node in graph.node_indices() {
        if !path.contains(&node) {
            let mut min_edges = graph
                .edges(node)
                .map(|edge| *edge.weight())
                .collect::<Vec<f64>>();
            min_edges.sort_by(|a, b| a.partial_cmp(b).unwrap());
            if min_edges.len() >= 2 {
                lb += (min_edges[0] + min_edges[1]) / 2.0;
            } else if min_edges.len() == 1 {
                lb += min_edges[0];
            }
        }
    }
    lb
}

fn tsp_helper<N: Clone + Debug>(
    graph: &UnGraph<N, f64>,
    current: NodeIndex,
    path: Vec<NodeIndex>,
    cost: f64,
    best_tour: &mut Vec<NodeIndex>,
    best_cost: &mut f64,
) {
    // Base case: If the current path has visited all the cities
    if path.len() == graph.node_count() {
        let last_edge = graph.find_edge(current, path[0]).unwrap();
        let total_cost = cost + graph[last_edge];

        if total_cost < *best_cost {
            *best_cost = total_cost;
            *best_tour = path.clone();
        }
        return;
    }

    // Recursive step: For each neighbor of the current city that has not been visited yet
    for neighbor in graph.neighbors(current) {
        if !path.contains(&neighbor) {
            let edge = graph.find_edge(current, neighbor).unwrap();
            let new_cost = cost + graph[edge];
            let mut new_path = path.clone();
            new_path.push(neighbor);

            let bound = new_cost + lower_bound(graph, &new_path);

            println!(
                "Path: {:?}\nCost: {}\nBound: {}\n",
                new_path
                    .iter()
                    .map(|n| graph.node_weight(*n).unwrap())
                    .collect::<Vec<_>>(),
                new_cost,
                bound
            );

            if bound < *best_cost {
                tsp_helper(graph, neighbor, new_path, new_cost, best_tour, best_cost);
            }
        }
    }
}
