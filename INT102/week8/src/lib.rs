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
    map_fn: fn(char) -> usize,
    gap_penalty: i32,
    scoring_matrix: &Vec<Vec<i32>>,
) -> (i32, Vec<(String, String)>, Vec<Vec<i32>>) {
    let seq1_len = seq1.len();
    let seq2_len = seq2.len();
    let mut dp_table = vec![vec![0; seq2_len + 1]; seq1_len + 1];

    for i in 0..=seq1_len {
        dp_table[i][0] = i as i32 * gap_penalty;
    }

    for j in 0..=seq2_len {
        dp_table[0][j] = j as i32 * gap_penalty;
    }

    for i in 1..=seq1_len {
        for j in 1..=seq2_len {
            let match_score = scoring_matrix[map_fn(seq1.chars().nth(i - 1).unwrap())]
                [map_fn(seq2.chars().nth(j - 1).unwrap())];
            let diagonal = dp_table[i - 1][j - 1] + match_score;
            let up = dp_table[i - 1][j] + gap_penalty;
            let left = dp_table[i][j - 1] + gap_penalty;

            dp_table[i][j] = diagonal.max(up).max(left);

            if dp_table[i][j] != 0 {
                let selected_name = if dp_table[i][j] == diagonal {
                    "diagonal"
                } else if dp_table[i][j] == up {
                    "up"
                } else {
                    "left"
                };

                println!("({}, {}) is {}", i + 1, j + 1, selected_name);
            }
        }
    }

    let score = dp_table[seq1_len][seq2_len];
    let alignments = traceback_global(
        seq1_len,
        seq2_len,
        seq1,
        seq2,
        map_fn,
        gap_penalty,
        scoring_matrix,
        &dp_table,
        "",
        "",
    );

    (score, alignments, dp_table)
}

fn traceback_global(
    i: usize,
    j: usize,
    seq1: &str,
    seq2: &str,
    map_fn: fn(char) -> usize,
    gap_penalty: i32,
    scoring_matrix: &Vec<Vec<i32>>,
    dp_table: &Vec<Vec<i32>>,
    aligned_seq1: &str,
    aligned_seq2: &str,
) -> Vec<(String, String)> {
    if i == 0 && j == 0 {
        return vec![(aligned_seq1.to_string(), aligned_seq2.to_string())];
    }

    let mut alignments = vec![];

    if i > 0
        && j > 0
        && dp_table[i][j]
            == dp_table[i - 1][j - 1]
                + scoring_matrix[map_fn(seq1.chars().nth(i - 1).unwrap())]
                    [map_fn(seq2.chars().nth(j - 1).unwrap())]
    {
        let mut new_seq1 = aligned_seq1.to_string();
        let mut new_seq2 = aligned_seq2.to_string();
        new_seq1.insert(0, seq1.chars().nth(i - 1).unwrap());
        new_seq2.insert(0, seq2.chars().nth(j - 1).unwrap());
        alignments.extend(traceback_global(
            i - 1,
            j - 1,
            seq1,
            seq2,
            map_fn,
            gap_penalty,
            scoring_matrix,
            dp_table,
            &new_seq1,
            &new_seq2,
        ));
    }
    if i > 0 && dp_table[i][j] == dp_table[i - 1][j] + gap_penalty {
        let mut new_seq1 = aligned_seq1.to_string();
        let mut new_seq2 = aligned_seq2.to_string();
        new_seq1.insert(0, seq1.chars().nth(i - 1).unwrap());
        new_seq2.insert(0, '-');
        alignments.extend(traceback_global(
            i - 1,
            j,
            seq1,
            seq2,
            map_fn,
            gap_penalty,
            scoring_matrix,
            dp_table,
            &new_seq1,
            &new_seq2,
        ));
    }
    if j > 0 && dp_table[i][j] == dp_table[i][j - 1] + gap_penalty {
        let mut new_seq1 = aligned_seq1.to_string();
        let mut new_seq2 = aligned_seq2.to_string();
        new_seq1.insert(0, '-');
        new_seq2.insert(0, seq2.chars().nth(j - 1).unwrap());
        alignments.extend(traceback_global(
            i,
            j - 1,
            seq1,
            seq2,
            map_fn,
            gap_penalty,
            scoring_matrix,
            dp_table,
            &new_seq1,
            &new_seq2,
        ));
    }

    alignments
}

pub fn local_alignment(
    seq1: &str,
    seq2: &str,
    map_fn: fn(char) -> usize,
    gap_penalty: i32,
    scoring_matrix: &Vec<Vec<i32>>,
) -> (i32, Vec<(String, String)>, Vec<Vec<i32>>) {
    let seq1_len = seq1.len();
    let seq2_len = seq2.len();
    let mut dp_table = vec![vec![0; seq2_len + 1]; seq1_len + 1];

    let mut max_value = 0;

    for i in 1..=seq1_len {
        for j in 1..=seq2_len {
            let match_score = scoring_matrix[map_fn(seq1.chars().nth(i - 1).unwrap())]
                [map_fn(seq2.chars().nth(j - 1).unwrap())];
            let diagonal = dp_table[i - 1][j - 1] + match_score;
            let up = dp_table[i - 1][j] + gap_penalty;
            let left = dp_table[i][j - 1] + gap_penalty;

            dp_table[i][j] = diagonal.max(up).max(left).max(0);

            if dp_table[i][j] != 0 {
                let selected_name = if dp_table[i][j] == diagonal {
                    "diagonal"
                } else if dp_table[i][j] == up {
                    "up"
                } else {
                    "left"
                };

                println!("({}, {}) is {}", i + 1, j + 1, selected_name);
            }

            if dp_table[i][j] > max_value {
                max_value = dp_table[i][j];
            }
        }
    }

    let mut alignments = vec![];
    for i in 1..=seq1_len {
        for j in 1..=seq2_len {
            if dp_table[i][j] == max_value {
                alignments.extend(traceback_local(
                    i,
                    j,
                    seq1,
                    seq2,
                    map_fn,
                    gap_penalty,
                    scoring_matrix,
                    &dp_table,
                    String::new(),
                    String::new(),
                ));
            }
        }
    }

    (max_value, alignments, dp_table)
}

fn traceback_local(
    i: usize,
    j: usize,
    seq1: &str,
    seq2: &str,
    map_fn: fn(char) -> usize,
    gap_penalty: i32,
    scoring_matrix: &Vec<Vec<i32>>,
    dp_table: &Vec<Vec<i32>>,
    aligned_seq1: String,
    aligned_seq2: String,
) -> Vec<(String, String)> {
    if dp_table[i][j] == 0 {
        return vec![(aligned_seq1, aligned_seq2)];
    }

    let mut alignments = vec![];

    let match_score = scoring_matrix[map_fn(seq1.chars().nth(i - 1).unwrap())]
        [map_fn(seq2.chars().nth(j - 1).unwrap())];

    if dp_table[i][j] == dp_table[i - 1][j - 1] + match_score {
        alignments.extend(traceback_local(
            i - 1,
            j - 1,
            seq1,
            seq2,
            map_fn,
            gap_penalty,
            scoring_matrix,
            dp_table,
            format!("{}{}", aligned_seq1, seq1.chars().nth(i - 1).unwrap()),
            format!("{}{}", aligned_seq2, seq2.chars().nth(j - 1).unwrap()),
        ));
    }

    if dp_table[i][j] == dp_table[i - 1][j] + gap_penalty {
        alignments.extend(traceback_local(
            i - 1,
            j,
            seq1,
            seq2,
            map_fn,
            gap_penalty,
            scoring_matrix,
            dp_table,
            format!("{}{}", aligned_seq1, seq1.chars().nth(i - 1).unwrap()),
            format!("{}{}", aligned_seq2, "-"),
        ));
    }

    if dp_table[i][j] == dp_table[i][j - 1] + gap_penalty {
        alignments.extend(traceback_local(
            i,
            j - 1,
            seq1,
            seq2,
            map_fn,
            gap_penalty,
            scoring_matrix,
            dp_table,
            format!("{}{}", aligned_seq1, "-"),
            format!("{}{}", aligned_seq2, seq2.chars().nth(j - 1).unwrap()),
        ));
    }

    alignments
}

pub fn nucleotide_to_index(c: char) -> usize {
    match c {
        'A' => 0,
        'C' => 1,
        'G' => 2,
        'T' => 3,
        _ => panic!("Invalid nucleotide character"),
    }
}
