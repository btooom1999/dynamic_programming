fn minimum_delete_sum(s1: String, s2: String) -> i32 {
    let (n1, n2) = (s1.len(), s2.len());
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();

    let mut dp = vec![vec![0; n1+1]; n2+1];
    for j in (0..n1).rev() {
        dp[n2][j] = s1[j] as i32 + dp[n2][j+1];
    }

    for i in (0..n2).rev() {
        dp[i][n1] = s2[i] as i32 + dp[i+1][n1];
    }

    for i in (0..n2).rev() {
        for j in (0..n1).rev() {
            dp[i][j] = dp[i+1][j+1];
            if s2[i] != s1[j] {
                dp[i][j] += s2[i] as i32 + s1[j] as i32;
                dp[i][j] = dp[i][j].min(s2[i] as i32 + dp[i+1][j]);
                dp[i][j] = dp[i][j].min(s1[j] as i32 + dp[i][j+1]);
            }
        }
    }

    dp[0][0]
}

pub fn main() {
    let s1 = "sea".to_string();
    let s2 = "eat".to_string();
    println!("{}", minimum_delete_sum(s1, s2));
}
