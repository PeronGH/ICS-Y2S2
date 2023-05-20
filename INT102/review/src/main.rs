mod brute_force;
mod divide_and_conquer;
mod dynamic_programming;
use brute_force::BruteForceSorting;
use petgraph::Graph;
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

    let haystack = b"Hello, world!";
    let needle = b"world";

    println!(
        "Linear Search: {:?}",
        brute_force::linear_search(haystack, needle)
    );

    let mut graph: Graph<(), ()> = Graph::new();

    let nodes: Vec<_> = (0..10).map(|_| graph.add_node(())).collect();

    graph.add_edge(nodes[0], nodes[1], ());
    graph.add_edge(nodes[0], nodes[2], ());
    graph.add_edge(nodes[1], nodes[3], ());
    graph.add_edge(nodes[1], nodes[4], ());
    graph.add_edge(nodes[2], nodes[5], ());
    graph.add_edge(nodes[2], nodes[6], ());
    graph.add_edge(nodes[4], nodes[7], ());
    graph.add_edge(nodes[4], nodes[8], ());
    graph.add_edge(nodes[7], nodes[9], ());

    print!("BFS: ");
    brute_force::bfs(&graph, nodes[0]);
    print!("\nDFS: ");
    brute_force::dfs(&graph, nodes[0], &mut HashSet::new());
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

    let adj_matrix = vec![
        vec![0, 5, std::i64::MAX, 10],
        vec![std::i64::MAX, 0, 3, std::i64::MAX],
        vec![std::i64::MAX, std::i64::MAX, 0, 1],
        vec![std::i64::MAX, std::i64::MAX, std::i64::MAX, 0],
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
