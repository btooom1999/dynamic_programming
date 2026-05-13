fn min_distance(word1: String, word2: String) -> i32 {
    let word1 = word1.as_bytes();
    let word2 = word2.as_bytes();
    let (m, n) = (word1.len(), word2.len());
    let mut dp = vec![vec![0; n+1]; m+1];

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if word1[i] == word2[j] {
                dp[i][j] = 1 + dp[i+1][j+1];
            } else {
                dp[i][j] = dp[i][j+1].max(dp[i+1][j]).max(dp[i+1][j+1]);
            }
        }
    }

    (word1.len() - dp[0][0] + word2.len() - dp[0][0]) as i32
}

pub fn main() {
    let word1 = "sea".to_string();
    let word2 = "eat".to_string();
    println!("{}", min_distance(word1, word2));
}
