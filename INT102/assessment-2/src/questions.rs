use petgraph::prelude::{DiGraph, UnGraph};

pub fn q1() {
    let pattern = "AGTAA";
    let shift_table = week7::horspool::create_shift_table(pattern);
    println!("{:?}", shift_table);

    let text = "AGCCGTGC";
    let result = week7::horspool::search(text, pattern);
    println!("{:?}", result);
}

pub fn q2() {
    let mut graph = DiGraph::<char, f64>::new();

    let a = graph.add_node('a');
    let b = graph.add_node('b');
    let c = graph.add_node('c');
    let d = graph.add_node('d');
    let e = graph.add_node('e');

    graph.add_edge(a, b, 4.);
    graph.add_edge(a, c, 5.);
    graph.add_edge(b, d, 8.);
    graph.add_edge(c, e, -8.);
    graph.add_edge(d, e, -6.);
    graph.add_edge(e, b, 2.);

    let result = week7::bellman_ford(&graph, a);

    println!("{:?}", result);
}

pub fn q3() {
    let str1 = "GAGT";
    let str2 = "AGACCT";

    let result = week8::lcs_tab(str1, str2);

    println!("{}", result);
}

pub fn q4() {
    let gap_penalty = -1;
    let scoring_matrix = vec![
        vec![1, -3, -2, -3],
        vec![-3, 1, -3, -2],
        vec![-2, -3, 1, -3],
        vec![-3, -2, -3, 1],
    ];

    let seq1 = "GAGT";
    let seq2 = "ACATGT";

    let (score_g, alignments_g) = week8::global_alignment(
        seq1,
        seq2,
        week8::nucleotide_to_index,
        gap_penalty,
        &scoring_matrix,
    );

    println!("Score: {}", score_g);
    println!("Alignments:");
    for (a1, a2) in alignments_g {
        println!("\n{}\n{}", a1, a2);
    }
    println!();

    let (score_l, alignments_l) = week8::local_alignment(
        seq1,
        seq2,
        week8::nucleotide_to_index,
        gap_penalty,
        &scoring_matrix,
    );

    println!("Score: {}", score_l);
    println!("Alignments:");
    for (a1, a2) in alignments_l {
        println!("\n{}\n{}", a1, a2);
    }
    println!();
}

pub fn q5() {
    let mut graph = UnGraph::<char, f64>::new_undirected();

    let nodes = vec![
        graph.add_node('a'),
        graph.add_node('b'),
        graph.add_node('c'),
        graph.add_node('d'),
        graph.add_node('e'),
    ];

    // this is a complete graph
    graph.extend_with_edges([
        (nodes[0], nodes[1], 4.),
        (nodes[0], nodes[2], 5.),
        (nodes[0], nodes[3], 2.),
        (nodes[0], nodes[4], 1.),
        (nodes[1], nodes[2], 4.),
        (nodes[1], nodes[3], 3.),
        (nodes[1], nodes[4], 1.),
        (nodes[2], nodes[3], 1.),
        (nodes[2], nodes[4], 8.),
        (nodes[3], nodes[4], 6.),
    ]);

    let result = week9::travelling_salesman(&graph, nodes[0]);

    println!("\nResult: {:?}", result);
}
