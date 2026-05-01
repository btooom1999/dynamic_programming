fn unique_paths(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut dp = vec![vec![0; n]; m];

    for i in (0..m).rev()  {
        for j in (0..n).rev() {
            if i == m-1 && j == n-1 {
                dp[i][j] = 1;
            }
            if i+1 != m {
                dp[i][j] += dp[i+1][j];
            }
            if j+1 != n {
                dp[i][j] += dp[i][j+1];
            }
        }
    }

    dp[0][0]
}

pub fn main() {
    let m = 3;
    let n = 7;
    println!("{:?}", unique_paths(m, n));
}
