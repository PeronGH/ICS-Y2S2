mod lcs;

fn main() {
    let str1 = "ABCBDAB";
    let str2 = "BDCABA";

    println!("LCS length: {}", lcs::lcs_tab(str1, str2));

    let m = str1.len();
    let n = str2.len();
    let mut memo = vec![vec![-1; n + 1]; m + 1];
    println!("LCS length: {}", lcs::lcs_memo(str1, str2, m, n, &mut memo));
}
