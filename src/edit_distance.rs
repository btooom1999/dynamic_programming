fn min_distance(word1: String, word2: String) -> i32 {
    let word1 = word1.as_bytes();
    let word2 = word2.as_bytes();
    let (m, n) = (word2.len(), word1.len());
    let mut dp = vec![vec![0; n+1]; m+1];

    dp[m] = (0..n+1).rev().collect::<Vec<_>>();
    for i in 0..m+1 {
        dp[m-i][n] = i;
    }

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if word2[i] == word1[j] {
                dp[i][j] = dp[i+1][j+1].min(1+dp[i+1][j].min(dp[i][j+1]));
            } else {
                dp[i][j] = 1+dp[i][j+1].min(dp[i+1][j]).min(dp[i+1][j+1]);
            }
        }
    }

    dp[0][0] as i32
}

pub fn main() {
    let word1 = "a".to_string();
    let word2 = "aaaa".to_string();
    println!("{}", min_distance(word1, word2));
}
