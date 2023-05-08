use std::fmt::Debug;

use petgraph::graph::{NodeIndex, UnGraph};

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

fn tsp_helper<N: Clone + Debug>(
    graph: &UnGraph<N, f64>,
    current: NodeIndex,
    path: Vec<NodeIndex>,
    cost: f64,
    best_tour: &mut Vec<NodeIndex>,
    best_cost: &mut f64,
) {
    // Base case: If the current path has visited all the cities
    // (i.e., the length of the path is equal to the number of cities in the graph),
    // the algorithm checks if the cost of the current tour (including the cost of returning to the starting city)
    // is less than the best cost found so far. If it is, the best tour and best cost are updated accordingly.
    if path.len() == graph.node_count() {
        let last_edge = graph.find_edge(current, path[0]).unwrap();
        let total_cost = cost + graph[last_edge];

        println!(
            "\nPath: {:?}",
            path.iter()
                .chain([path[0]].iter())
                .map(|i| graph.node_weight(*i).unwrap())
                .collect::<Vec<_>>()
        );
        println!("Cost: {:?}\n", total_cost);

        if total_cost < *best_cost {
            *best_cost = total_cost;
            *best_tour = path.clone();
        }
        return;
    }

    // Recursive step: For each neighbor of the current city that has not been visited yet (i.e., not in the current path),
    // the algorithm calculates the cost of moving to that neighbor. If the new cost is less than the current best cost,
    // it proceeds with the recursion by calling the tsp function again with the neighbor as the new current city,
    // an updated path that includes the neighbor, and the new cost.
    for neighbor in graph.neighbors(current) {
        if !path.contains(&neighbor) {
            let edge = graph.find_edge(current, neighbor).unwrap();
            let new_cost = cost + graph[edge];
            if new_cost < *best_cost {
                let mut new_path = path.clone();
                new_path.push(neighbor);

                println!(
                    "\nPath: {:?}",
                    new_path
                        .iter()
                        .map(|i| graph.node_weight(*i).unwrap())
                        .collect::<Vec<_>>()
                );
                println!("Cost: {:?}", new_cost);

                tsp_helper(graph, neighbor, new_path, new_cost, best_tour, best_cost);
            }
        }
    }
}
