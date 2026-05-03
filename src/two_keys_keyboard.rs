fn min_steps(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![vec![0; n+1]; n+1];

    for i in 1..n+1 {
        let mut min = i32::MAX;
        for j in 1..i+1 {
            if i == j {
                dp[i][j] = if i == 1 { 1 } else { min+1 }
            } else if dp[i-j][j] != 0 {
                dp[i][j] = dp[i-j][j] + 1;
                min = min.min(dp[i][j]);
            }
        }
    }

    dp[n][n]-1
}

pub fn main() {
    let n = 9;
    println!("{:?}", min_steps(n));
}
