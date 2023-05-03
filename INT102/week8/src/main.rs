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
}
