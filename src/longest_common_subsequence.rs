fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let text1 = text1.as_bytes();
    let text2 = text2.as_bytes();
    let (m, n) = (text1.len(), text2.len());

    let mut dp = vec![vec![0; n+1]; m+1];

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if text1[i] == text2[j] {
                dp[i][j] = 1 + dp[i+1][j+1];
            } else {
                dp[i][j] = dp[i+1][j].max(dp[i][j+1]);
            }
        }
    }

    dp[0][0]
}

pub fn main() {
    let text1 = "abcde".to_string();
    let text2 = "ace".to_string();
    println!("{}", longest_common_subsequence(text1, text2));
}
