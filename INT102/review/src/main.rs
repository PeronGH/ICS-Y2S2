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

    let str1 = "APPLE";
    let str2 = "PP";

    println!(
        "Longest Common Subsequence: {}",
        dynamic_programming::lcs(str1, str2)
    )
}
