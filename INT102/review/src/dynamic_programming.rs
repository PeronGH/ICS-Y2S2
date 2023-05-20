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

fn backtrack_lcs(dp: &Vec<Vec<usize>>, str1: &str, str2: &str, i: usize, j: usize) -> String {
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
