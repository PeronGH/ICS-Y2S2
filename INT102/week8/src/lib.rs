use std::cmp::{max, min};

pub fn lcs_tab(str1: &str, str2: &str) -> (usize, Vec<Vec<usize>>) {
    let m = str1.chars().count();
    let n = str2.chars().count();
    // Create a 2D table to store the lengths of LCS for all subproblems
    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Iterate through the strings, updating the DP table
    for i in 1..=m {
        for j in 1..=n {
            // If the characters at the current positions match,
            // the LCS length is one greater than the length of the LCS
            // for the previous characters in both strings
            // (indicated by the ↖ arrow)
            if str1.chars().nth(i - 1) == str2.chars().nth(j - 1) {
                dp[i][j] = 1 + dp[i - 1][j - 1];
            } else {
                // If the characters at the current positions do not match,
                // the LCS length is the maximum of the lengths of the LCS
                // for the previous character in the first string (indicated by the ↑ arrow)
                // and the previous character in the second string (indicated by the ← arrow)
                dp[i][j] = usize::max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }
    // The value in the bottom-right cell of the DP table (dp[m][n])
    // represents the length of the LCS for the entire input strings
    (dp[m][n], dp)
}

pub fn lcs_memo(str1: &str, str2: &str, m: usize, n: usize, memo: &mut Vec<Vec<isize>>) -> usize {
    // Base case: If either string is empty, the LCS length is 0
    if m == 0 || n == 0 {
        return 0;
    }

    // If the memoization table contains a non-negative value,
    // the subproblem has already been solved, and we can return the result directly
    if memo[m][n] != -1 {
        return memo[m][n] as usize;
    }

    // If the characters at the current positions match,
    // the LCS length is one greater than the length of the LCS
    // for the previous characters in both strings
    if str1.chars().nth(m - 1) == str2.chars().nth(n - 1) {
        memo[m][n] = (1 + lcs_memo(str1, str2, m - 1, n - 1, memo)) as isize;
    } else {
        // If the characters at the current positions do not match,
        // the LCS length is the maximum of the lengths of the LCS
        // for the previous character in the first string
        // and the previous character in the second string
        memo[m][n] = usize::max(
            lcs_memo(str1, str2, m - 1, n, memo),
            lcs_memo(str1, str2, m, n - 1, memo),
        ) as isize;
    }

    // Store the calculated LCS length in the memoization table and return the result
    memo[m][n] as usize
}

pub fn lcs_brute_force(str1: &str, str2: &str) -> usize {
    if str1.is_empty() || str2.is_empty() {
        // Base case: If either of the strings is empty, the length of LCS is 0
        0
    } else if str1.chars().nth(0) == str2.chars().nth(0) {
        // If the first characters of both strings are the same, increment LCS length and continue the search
        1 + lcs_brute_force(&str1[1..], &str2[1..])
    } else {
        // If the first characters don't match, find the maximum length of LCS by considering both possibilities
        lcs_brute_force(&str1[1..], &str2).max(lcs_brute_force(&str1, &str2[1..]))
    }
}

pub fn global_alignment(
    seq1: &str,
    seq2: &str,
    gap_penalty: i32,
    scoring_matrix: &Vec<Vec<i32>>,
) -> (i32, String, String) {
    let mut max_score = std::i32::MIN;
    let mut aligned_seq1 = String::new();
    let mut aligned_seq2 = String::new();

    let sub_seq_pairs = seq1.chars().enumerate().flat_map(|(i, _)| {
        seq2.chars()
            .enumerate()
            .map(move |(j, _)| (&seq1[0..=i], &seq2[0..=j]))
    });

    for (sub_seq1, sub_seq2) in sub_seq_pairs {
        let score = alignment_score(sub_seq1, sub_seq2, gap_penalty, &scoring_matrix);

        if score > max_score {
            max_score = score;
            aligned_seq1 = sub_seq1.to_string();
            aligned_seq2 = sub_seq2.to_string();
        }
    }

    (max_score, aligned_seq1, aligned_seq2)
}

pub fn local_alignment(
    seq1: &str,
    seq2: &str,
    gap_penalty: i32,
    scoring_matrix: &Vec<Vec<i32>>,
) -> (i32, String, String) {
    let mut max_score = i32::MIN;
    let mut aligned_seq1 = String::new();
    let mut aligned_seq2 = String::new();

    let sub_seq_pairs = seq1.chars().enumerate().flat_map(|(i, _)| {
        seq2.chars().enumerate().flat_map(move |(j, _)| {
            (i..seq1.len())
                .flat_map(move |k| (j..seq2.len()).map(move |l| (&seq1[i..=k], &seq2[j..=l])))
        })
    });

    for (sub_seq1, sub_seq2) in sub_seq_pairs {
        let score = alignment_score(sub_seq1, sub_seq2, gap_penalty, &scoring_matrix);

        if score > max_score {
            max_score = score;
            aligned_seq1 = sub_seq1.to_string();
            aligned_seq2 = sub_seq2.to_string();
        }
    }

    (max_score, aligned_seq1, aligned_seq2)
}

fn alignment_score(
    seq1: &str,
    seq2: &str,
    gap_penalty: i32,
    scoring_matrix: &Vec<Vec<i32>>,
) -> i32 {
    let mut score = 0;

    let shorter_len = min(seq1.len(), seq2.len());
    let longer_len = max(seq1.len(), seq2.len());

    for i in 0..shorter_len {
        let char1 = nucleotide_to_index(seq1.chars().nth(i).unwrap());
        let char2 = nucleotide_to_index(seq2.chars().nth(i).unwrap());
        score += scoring_matrix[char1][char2];
    }

    score += (longer_len - shorter_len) as i32 * gap_penalty;

    score
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
