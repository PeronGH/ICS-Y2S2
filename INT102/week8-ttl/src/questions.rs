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

pub fn q2() {
    let mut arr = [2, 3, 4, 3, 2, 1, 1, 2];
    week7::counting_sort(&mut arr);
    println!("Q2: {:?}", arr);
}

pub fn q3() {
    let pattern = "TCCTATTCTT";
    let table = week7::horspool::create_shift_table(pattern);

    println!("Q3A: {:?}", table);

    let text = "TTATAGATCTGGTATTCTTTTATAGATCTCCTATTCTT";
    let result = week7::horspool::search(text, pattern);

    println!("Q3B: {:?}", result);
}

pub fn q4() {
    let seq1 = "AATG";
    let seq2 = "AGC";
    let gap_penalty = -5;
    let scoring_matrix = vec![
        vec![2, -7, -5, -7],
        vec![-7, 2, -7, -5],
        vec![-5, -7, 2, -7],
        vec![-7, -5, -7, 2],
    ];

    let (score_g, seq_g) = week8::global_alignment(
        seq1,
        seq2,
        nucleotide_to_index,
        gap_penalty,
        &scoring_matrix,
    );
    println!("Score: {}", score_g);
    println!("Aligned Sequences: {:?}", seq_g);

    let (score_l, seq_l) = week8::local_alignment(
        seq1,
        seq2,
        nucleotide_to_index,
        gap_penalty,
        &scoring_matrix,
    );
    println!("Score: {}", score_l);
    println!("Aligned Sequence: {:?}", seq_l);
}

fn nucleotide_to_index(c: char) -> usize {
    match c {
        'A' => 0,
        'C' => 1,
        'G' => 2,
        'T' => 3,
        _ => panic!("Invalid nucleotide character"),
    }
}
