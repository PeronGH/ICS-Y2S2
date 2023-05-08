use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::EdgeType;
use std::f64;
use std::fmt::Display;

// Function to compute the shortest path from the source node to all other nodes in the graph.
// It returns an Option<Vec<f64>> containing the shortest path distances.
// If a negative cycle is detected, it returns None.
pub fn bellman_ford<N, E>(graph: &Graph<N, f64, E>, source: NodeIndex) -> Option<Vec<f64>>
where
    N: Clone + Display,
    E: EdgeType,
{
    // Get the number of vertices in the graph.
    let num_vertices = graph.node_count();

    // Initialize the distance vector with infinity for all nodes, except the source node.
    let mut distances = vec![f64::INFINITY; num_vertices];

    // Set the distance to the source node as 0.
    distances[source.index()] = 0.0;

    // Initialize the previous node record
    let mut prev = vec![None; num_vertices];

    // Relax all edges (num_vertices - 1) times, to guarantee that the shortest paths are found.
    for i in 0..num_vertices - 1 {
        println!("Iteration {}:", i + 1);
        // Iterate through all edges in the graph.
        for edge in graph.edge_references() {
            // Get the source, target nodes (u, v), and the weight of the edge.
            let (u, v, weight) = (edge.source(), edge.target(), *edge.weight());

            // If the current distance to the target node (v) can be improved by going through the source node (u),
            // update the distance and the previous node record.
            if distances[u.index()] + weight < distances[v.index()] {
                distances[v.index()] = distances[u.index()] + weight;
                prev[v.index()] = Some(u);

                // Print the updated table
                println!("Vertex | Distance | Previous Node");
                for i in 0..num_vertices {
                    let vertex_name = graph.node_weight(NodeIndex::new(i)).unwrap();
                    let distance = distances[i];
                    let prev_node = prev[i].map_or("None".to_string(), |node| {
                        format!("{}", graph.node_weight(node).unwrap())
                    });
                    println!("{:<6} | {:<8} | {}", vertex_name, distance, prev_node);
                }
            } else {
                println!(
                    "No update for edge ({}, {})",
                    graph.node_weight(u).unwrap(),
                    graph.node_weight(v).unwrap()
                )
            }
            println!();
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
pub fn floyd_warshall<N, E>(graph: &Graph<N, f64, E>) -> Option<Vec<Vec<f64>>>
where
    N: Clone,
    E: EdgeType,
{
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

pub fn counting_sort(arr: &mut [usize]) {
    // Step 1: Find the maximum value in the input array
    let max_val = match arr.iter().max() {
        Some(&max) => max,
        None => return,
    };

    // Step 2: Create count array
    let mut count_arr = vec![0; max_val + 1];

    // Step 3: Count occurrences of each number
    for &num in arr.iter() {
        count_arr[num] += 1;
    }

    // Step 4: Compute cumulative sum
    for i in 1..count_arr.len() {
        count_arr[i] += count_arr[i - 1];
    }

    // Step 5: Create output array
    let mut output_arr = vec![0; arr.len()];

    // Step 6: Place numbers into output array
    for &num in arr.iter().rev() {
        output_arr[count_arr[num] - 1] = num;
        count_arr[num] -= 1;
    }

    // Step 7: Copy sorted elements back to input array
    arr.clone_from_slice(&output_arr);
}

pub mod horspool {
    use std::collections::HashMap;

    // Step 1: Create shift table
    pub fn create_shift_table(pattern: &str) -> HashMap<char, usize> {
        let m = pattern.len();
        pattern
            .chars()
            .enumerate()
            .take(m - 1)
            .map(|(i, ch)| (ch, m - i - 1)) // Step 1.1: Compute the shift for each character in the pattern
            .collect() // Step 1.2: Collect shifts into a HashMap
    }

    pub fn search(text: &str, pattern: &str) -> (Option<usize>, usize) {
        let (n, m) = (text.len(), pattern.len());
        if m > n {
            return (None, 0);
        }

        // Step 2: Generate the shift table for the given pattern
        let shift_table = create_shift_table(pattern);

        // Step 3: Initialize skip and comparison count variables
        let mut skip = 0;
        let mut cmp_count = 0;

        // Step 4: Iterate through the text, comparing with the pattern
        while skip <= n - m {
            cmp_count += 1;

            println!("{}\n{}", text, " ".repeat(skip) + pattern);

            // Step 4.1: Check if the pattern starts at the current position in the text
            if text
                .chars()
                .skip(skip)
                .collect::<String>()
                .starts_with(pattern)
            {
                // Step 4.1.1: If the pattern is found, return the position and comparison count
                return (Some(skip), cmp_count);
            }

            // Step 4.2: Update the skip value based on the shift table
            skip += shift_table
                .get(&text.chars().skip(skip).nth(m - 1).unwrap())
                .unwrap_or(&m);
        }

        // Step 5: If the pattern is not found, return None and the comparison count
        (None, cmp_count)
    }
}
