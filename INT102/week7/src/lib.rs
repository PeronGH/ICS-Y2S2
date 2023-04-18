use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::EdgeRef;
use std::f64;

// Function to compute the shortest path from the source node to all other nodes in the graph.
// It returns an Option<Vec<f64>> containing the shortest path distances.
// If a negative cycle is detected, it returns None.
pub fn bellman_ford(graph: &UnGraph<usize, f64>, source: NodeIndex) -> Option<Vec<f64>> {
    // Get the number of vertices in the graph.
    let num_vertices = graph.node_count();

    // Initialize the distance vector with infinity for all nodes, except the source node.
    let mut distances = vec![f64::INFINITY; num_vertices];

    // Set the distance to the source node as 0.
    distances[source.index()] = 0.0;

    // Relax all edges (num_vertices - 1) times, to guarantee that the shortest paths are found.
    for _ in 0..num_vertices - 1 {
        // Iterate through all edges in the graph.
        for edge in graph.edge_references() {
            // Get the source, target nodes (u, v), and the weight of the edge.
            let (u, v, weight) = (edge.source(), edge.target(), *edge.weight());

            // If the current distance to the target node (v) can be improved by going through the source node (u),
            // update the distance.
            if distances[u.index()] + weight < distances[v.index()] {
                distances[v.index()] = distances[u.index()] + weight;
            }
        }
    }

    // Check for negative cycles by iterating through all edges again.
    for edge in graph.edge_references() {
        // Get the source, target nodes (u, v), and the weight of the edge.
        let (u, v, weight) = (edge.source(), edge.target(), *edge.weight());

        // If the distance to the target node (v) can still be improved by going through the source node (u),
        // it means a negative cycle exists in the graph.
        if distances[u.index()] + weight < distances[v.index()] {
            return None; // Negative cycle detected, return None.
        }
    }

    // If no negative cycles are detected, return the computed shortest path distances.
    Some(distances)
}

// Function to compute the shortest paths between all pairs of nodes in a directed, weighted graph.
// If a negative cycle is detected, it returns None.
// Otherwise, it returns a two-dimensional Vec containing the shortest path distances between all pairs of nodes.
pub fn floyd_warshall(graph: &UnGraph<usize, f64>) -> Option<Vec<Vec<f64>>> {
    // Get the number of vertices in the graph.
    let num_vertices = graph.node_count();

    // Initialize the distance matrix with infinity for all node pairs,
    // except for direct edges and diagonal elements.
    let mut dist = vec![vec![f64::INFINITY; num_vertices]; num_vertices];

    // Initialize the distance matrix with direct edge weights and set the diagonal elements to 0.
    for node in graph.node_indices() {
        dist[node.index()][node.index()] = 0.0;
        for edge in graph.edges(node) {
            let target = edge.target().index();
            dist[node.index()][target] = *edge.weight();
        }
    }

    // Perform the Floyd-Warshall algorithm by iterating through all nodes as intermediate nodes (k).
    for k in 0..num_vertices {
        // For each pair of nodes (i, j), check if the distance can be improved by going through node k.
        for i in 0..num_vertices {
            for j in 0..num_vertices {
                // Calculate the distance from node i to node j through node k.
                let new_dist = dist[i][k] + dist[k][j];

                // If the new distance is less than the current distance from node i to node j,
                // update the distance in the matrix.
                if new_dist < dist[i][j] {
                    dist[i][j] = new_dist;
                }
            }
        }
    }

    // Check for negative cycles by looking at the diagonal elements of the distance matrix.
    for i in 0..num_vertices {
        if dist[i][i] < 0.0 {
            return None; // Negative cycle detected, return None.
        }
    }

    // If no negative cycles are detected, return the computed shortest path distances matrix.
    Some(dist)
}
