pub fn lcs_tab(str1: &str, str2: &str) -> usize {
    let m = str1.len();
    let n = str2.len();
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
    dp[m][n]
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
