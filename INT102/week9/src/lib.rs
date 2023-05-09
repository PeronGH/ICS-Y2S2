use petgraph::algo::min_spanning_tree;
use petgraph::data::Element;
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

    println!("Initial Bound: {}\n", lower_bound(graph, &path));

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
    // Create a new graph that only includes the unvisited nodes
    let unvisited_graph = graph.clone().filter_map(
        |i, nw| {
            if path.contains(&i) {
                None
            } else {
                Some(nw.clone())
            }
        },
        |_, ew| Some(*ew),
    );

    // Calculate the minimum spanning tree of the unvisited nodes
    let mst = min_spanning_tree(&unvisited_graph);

    // The lower bound is the sum of the edge weights in the minimum spanning tree
    let lb: f64 = mst
        .filter_map(|element| match element {
            Element::Edge { weight, .. } => Some(weight),
            _ => None,
        })
        .sum();

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
