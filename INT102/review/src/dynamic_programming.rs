use std::f64::INFINITY;

use petgraph::prelude::*;

pub fn lcs(str1: &str, str2: &str) -> String {
    let m = str1.len();
    let n = str2.len();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if str1.chars().nth(i - 1) == str2.chars().nth(j - 1) {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    backtrack_lcs(&dp, str1, str2, m, n)
}

fn backtrack_lcs<T: Ord>(dp: &Vec<Vec<T>>, str1: &str, str2: &str, i: usize, j: usize) -> String {
    if i == 0 || j == 0 {
        String::new()
    } else if str1.chars().nth(i - 1) == str2.chars().nth(j - 1) {
        format!(
            "{}{}",
            backtrack_lcs(dp, str1, str2, i - 1, j - 1),
            str1.chars().nth(i - 1).unwrap()
        )
    } else {
        if dp[i - 1][j] > dp[i][j - 1] {
            backtrack_lcs(dp, str1, str2, i - 1, j)
        } else {
            backtrack_lcs(dp, str1, str2, i, j - 1)
        }
    }
}

pub fn global_alignment(
    str1: &str,
    str2: &str,
    gap_penalty: i64,
    score: fn(char, char) -> i64,
) -> (i64, (String, String)) {
    let m = str1.len();
    let n = str2.len();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    // Initialize the DP table
    for i in 0..=m {
        dp[i][0] = i as i64 * gap_penalty;
    }
    for j in 0..=n {
        dp[0][j] = j as i64 * gap_penalty;
    }

    // Fill the DP table
    for i in 1..=m {
        for j in 1..=n {
            let match_score = dp[i - 1][j - 1]
                + score(
                    str1.chars().nth(i - 1).unwrap(),
                    str2.chars().nth(j - 1).unwrap(),
                );
            let delete = dp[i - 1][j] + gap_penalty;
            let insert = dp[i][j - 1] + gap_penalty;
            dp[i][j] = match_score.max(delete).max(insert);
        }
    }

    // Backtrack to find the optimal alignment
    let alignment = backtrack_global_alignment(&dp, str1, str2, m, n, gap_penalty, score);

    (dp[m][n], alignment)
}

fn backtrack_global_alignment(
    dp: &Vec<Vec<i64>>,
    str1: &str,
    str2: &str,
    i: usize,
    j: usize,
    gap_penalty: i64,
    score: fn(char, char) -> i64,
) -> (String, String) {
    if i == 0 && j == 0 {
        (String::new(), String::new())
    } else if i > 0 && dp[i][j] == dp[i - 1][j] + gap_penalty {
        let (x, y) = backtrack_global_alignment(dp, str1, str2, i - 1, j, gap_penalty, score);
        (
            format!("{}{}", x, str1.chars().nth(i - 1).unwrap()),
            format!("{}-", y),
        )
    } else if j > 0 && dp[i][j] == dp[i][j - 1] + gap_penalty {
        let (x, y) = backtrack_global_alignment(dp, str1, str2, i, j - 1, gap_penalty, score);
        (
            format!("{}-", x),
            format!("{}{}", y, str2.chars().nth(j - 1).unwrap()),
        )
    } else {
        let (x, y) = backtrack_global_alignment(dp, str1, str2, i - 1, j - 1, gap_penalty, score);
        (
            format!("{}{}", x, str1.chars().nth(i - 1).unwrap()),
            format!("{}{}", y, str2.chars().nth(j - 1).unwrap()),
        )
    }
}

pub fn local_alignment(
    str1: &str,
    str2: &str,
    gap_penalty: i64,
    score: fn(char, char) -> i64,
) -> (i64, (String, String)) {
    let m = str1.len();
    let n = str2.len();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    let mut max_i = 0;
    let mut max_j = 0;
    let mut max_score = 0;

    // Fill the DP table
    for i in 1..=m {
        for j in 1..=n {
            let match_score = dp[i - 1][j - 1]
                + score(
                    str1.chars().nth(i - 1).unwrap(),
                    str2.chars().nth(j - 1).unwrap(),
                );
            let delete = dp[i - 1][j] + gap_penalty;
            let insert = dp[i][j - 1] + gap_penalty;
            dp[i][j] = 0.max(match_score.max(delete).max(insert));

            if dp[i][j] > max_score {
                max_score = dp[i][j];
                max_i = i;
                max_j = j;
            }
        }
    }

    // Backtrack to find the optimal alignment
    let alignment = backtrack_local_alignment(&dp, str1, str2, max_i, max_j, gap_penalty, score);

    (max_score, alignment)
}

fn backtrack_local_alignment(
    dp: &Vec<Vec<i64>>,
    str1: &str,
    str2: &str,
    i: usize,
    j: usize,
    gap_penalty: i64,
    score: fn(char, char) -> i64,
) -> (String, String) {
    if dp[i][j] == 0 {
        (String::new(), String::new())
    } else if i > 0 && dp[i][j] == dp[i - 1][j] + gap_penalty {
        let (x, y) = backtrack_local_alignment(dp, str1, str2, i - 1, j, gap_penalty, score);
        (
            format!("{}{}", x, str1.chars().nth(i - 1).unwrap()),
            format!("{}-", y),
        )
    } else if j > 0 && dp[i][j] == dp[i][j - 1] + gap_penalty {
        let (x, y) = backtrack_local_alignment(dp, str1, str2, i, j - 1, gap_penalty, score);
        (
            format!("{}-", x),
            format!("{}{}", y, str2.chars().nth(j - 1).unwrap()),
        )
    } else {
        let (x, y) = backtrack_local_alignment(dp, str1, str2, i - 1, j - 1, gap_penalty, score);
        (
            format!("{}{}", x, str1.chars().nth(i - 1).unwrap()),
            format!("{}{}", y, str2.chars().nth(j - 1).unwrap()),
        )
    }
}

pub fn bellman_ford<N>(graph: &Graph<N, f64>, source: NodeIndex) -> Option<Vec<f64>> {
    let n = graph.node_count();
    let mut dist = vec![INFINITY; n];

    let source_index = source.index();

    // Distance from source to itself is 0
    dist[source_index] = 0.0;

    // Relax edges repeatedly
    for _ in 0..n - 1 {
        for edge in graph.edge_references() {
            let (u, v, weight) = (edge.source().index(), edge.target().index(), *edge.weight());

            if dist[u] + weight < dist[v] {
                dist[v] = dist[u] + weight;
            }
        }
    }

    // Check for negative weight cycles
    for edge in graph.edge_references() {
        let (u, v, weight) = (edge.source().index(), edge.target().index(), *edge.weight());

        if dist[u] + weight < dist[v] {
            // Graph contains negative weight cycle
            return None;
        }
    }

    Some(dist)
}

pub fn floyd_warshall(adj_matrix: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let n = adj_matrix.len();
    let mut dist = adj_matrix.clone();

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] != i64::MAX && dist[k][j] != i64::MAX {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
    }

    dist
}

pub fn assembly_line_scheduling(
    a: (&[u64], &[u64]),
    t: (&[u64], &[u64]),
    e: (u64, u64),
    x: (u64, u64),
) -> u64 {
    let n = a.0.len();
    let mut f = (vec![0; n], vec![0; n]);

    f.0[0] = e.0 + a.0[0];
    f.1[0] = e.1 + a.1[0];

    for i in 1..n {
        f.0[i] = (f.0[i - 1] + a.0[i]).min(f.1[i - 1] + t.1[i - 1] + a.0[i]);
        f.1[i] = (f.1[i - 1] + a.1[i]).min(f.0[i - 1] + t.0[i - 1] + a.1[i]);
    }

    (f.0[n - 1] + x.0).min(f.1[n - 1] + x.1)
}
