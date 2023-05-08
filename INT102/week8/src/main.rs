use week8::*;

fn main() {
    let str1 = "ABCBDAB";
    let str2 = "BDCABA";

    println!("LCS length: {}", lcs_brute_force(str1, str2));
    println!("LCS length: {:?}", lcs_tab(str1, str2));

    let m = str1.chars().count();
    let n = str2.chars().count();
    let mut memo = vec![vec![-1; n + 1]; m + 1];
    println!("LCS length: {}", lcs_memo(str1, str2, m, n, &mut memo));

    let gap_penalty = -5;
    let scoring_matrix = vec![
        vec![2, -7, -5, -7],
        vec![-7, 2, -7, -5],
        vec![-5, -7, 2, -7],
        vec![-7, -5, -7, 2],
    ];

    let seq1 = "AGC";
    let seq2 = "AATG";

    let (score_g, alignments_g) = global_alignment(
        seq1,
        seq2,
        nucleotide_to_index,
        gap_penalty,
        &scoring_matrix,
    );

    println!("Score: {}", score_g);
    println!("Alignments: {:?}", alignments_g);

    let (score_l, alignments_l) = local_alignment(
        seq1,
        seq2,
        nucleotide_to_index,
        gap_penalty,
        &scoring_matrix,
    );

    println!("Score: {}", score_l);
    println!("Alignments: {:?}", alignments_l);
}
