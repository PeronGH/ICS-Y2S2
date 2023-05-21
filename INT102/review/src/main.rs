mod brute_force;
mod divide_and_conquer;
mod dynamic_programming;
mod greedy;
mod space_for_time;
use brute_force::BruteForceSorting;
use petgraph::prelude::*;
use std::collections::HashSet;

fn main() {
    let unsorted_arr = [7, 5, 3, 2, 6, 8, 9, 4, 2];

    // Brute Force

    println!(
        "After Selection Sort: {:?}",
        unsorted_arr.clone().selection_sort()
    );

    println!(
        "After Bubble Sort: {:?}",
        unsorted_arr.clone().bubble_sort()
    );

    println!(
        "After Insertion Sort: {:?}",
        unsorted_arr.clone().insertion_sort()
    );

    let haystack = "Hello, world!";
    let needle = "world";

    println!(
        "Linear Search: {:?}",
        brute_force::linear_search(haystack.as_bytes(), needle.as_bytes())
    );

    let mut tree: Graph<(), ()> = Graph::new();

    let nodes: Vec<_> = (0..10).map(|_| tree.add_node(())).collect();

    tree.add_edge(nodes[0], nodes[1], ());
    tree.add_edge(nodes[0], nodes[2], ());
    tree.add_edge(nodes[1], nodes[3], ());
    tree.add_edge(nodes[1], nodes[4], ());
    tree.add_edge(nodes[2], nodes[5], ());
    tree.add_edge(nodes[2], nodes[6], ());
    tree.add_edge(nodes[4], nodes[7], ());
    tree.add_edge(nodes[4], nodes[8], ());
    tree.add_edge(nodes[7], nodes[9], ());

    print!("BFS: ");
    brute_force::bfs(&tree, nodes[0]);
    print!("\nDFS: ");
    brute_force::dfs(&tree, nodes[0], &mut HashSet::new());
    println!();

    // Divide and Conquer

    let sorted_arr = divide_and_conquer::merge_sort(&unsorted_arr);
    println!("After Merge Sort: {:?}", sorted_arr);

    println!(
        "Binary Search for 9: {:?}",
        divide_and_conquer::binary_search(&sorted_arr, &9)
    );

    println!(
        "Binary Search for 11: {:?}",
        divide_and_conquer::binary_search(&sorted_arr, &11)
    );

    // Dynamic Programming

    let str1 = "GAGT";
    let str2 = "ACATGT";

    println!(
        "Longest Common Subsequence: {}",
        dynamic_programming::lcs(str1, str2)
    );

    let gap_penalty = -1;

    let (score_global, (align_str1_global, align_str2_global)) =
        dynamic_programming::global_alignment(str1, str2, gap_penalty, score_fn);

    println!("Global Score: {}", score_global);
    println!("Global Alignment 1: {}", align_str1_global);
    println!("Global Alignment 2: {}", align_str2_global);

    let (score_local, (align_str1_local, align_str2_local)) =
        dynamic_programming::local_alignment(str1, str2, gap_penalty, score_fn);

    println!("Local Score: {}", score_local);
    println!("Local Alignment 1: {}", align_str1_local);
    println!("Local Alignment 2: {}", align_str2_local);

    let mut graph = Graph::<_, f64>::new();

    let node1 = graph.add_node("node1");
    let node2 = graph.add_node("node2");
    let node3 = graph.add_node("node3");
    let node4 = graph.add_node("node4");
    let node5 = graph.add_node("node5");

    graph.add_edge(node1, node2, 5.);
    graph.add_edge(node1, node3, 4.);
    graph.add_edge(node2, node3, -2.);
    graph.add_edge(node3, node4, 3.);
    graph.add_edge(node2, node5, 6.);
    graph.add_edge(node4, node5, 2.);

    let shortest_distances = dynamic_programming::bellman_ford(&graph, node1);

    match shortest_distances {
        Some(distances) => {
            for (node, &dist) in graph.node_indices().zip(distances.iter()) {
                println!("Distance from node1 to {} = {}", graph[node], dist);
            }
        }
        None => println!("Graph contains a negative cycle"),
    }

    let adj_matrix = vec![
        vec![0, 5, 4, i64::MAX, i64::MAX],               // edges from node1
        vec![i64::MAX, 0, -2, i64::MAX, 6],              // edges from node2
        vec![i64::MAX, i64::MAX, 0, 3, i64::MAX],        // edges from node3
        vec![i64::MAX, i64::MAX, i64::MAX, 0, 2],        // edges from node4
        vec![i64::MAX, i64::MAX, i64::MAX, i64::MAX, 0], // edges from node5
    ];

    for row in &dynamic_programming::floyd_warshall(&adj_matrix) {
        println!("{:?}", row);
    }

    let a = (&[7, 9, 3, 4, 8, 4][..], &[8, 5, 6, 4, 5, 7][..]);
    let t = (&[2, 3, 1, 3, 4][..], &[2, 1, 2, 2, 1][..]);
    let e = (2, 4);
    let x = (3, 2);

    println!(
        "Minimum time by Assembly Line Scheduling Algorithm: {}",
        dynamic_programming::assembly_line_scheduling(a, t, e, x)
    );

    // Space/Time

    println!(
        "After Counting Sort: {:?}",
        space_for_time::counting_sort(&unsorted_arr)
    );

    println!(
        "Horspool Search: {:?}",
        space_for_time::horspool_search(haystack, needle)
    )
}

fn score_fn(x: char, y: char) -> i64 {
    SCORING_MATRIX[week8::nucleotide_to_index(x)][week8::nucleotide_to_index(y)]
}

const SCORING_MATRIX: [[i64; 4]; 4] = [
    [1, -3, -2, -3],
    [-3, 1, -3, -2],
    [-2, -3, 1, -3],
    [-3, -2, -3, 1],
];
