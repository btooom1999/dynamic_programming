fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
    let (n, k, max_pts) = (n as usize, k as usize, max_pts as usize);
    let mut dp = vec![0f64; n + max_pts + 2];
    for i in (0..=n).rev() {
        if i >= k {
            dp[i] = 1.0;
        } else {
            dp[i] = (dp[i+1] - dp[i+max_pts+1]) / max_pts as f64;
        }
        dp[i] += dp[i+1];
    }

    dp[0] - dp[1]
}

pub fn main() {
    let n = 6;
    let k = 3;
    let max_pts = 10;
    println!("{}", new21_game(n, k, max_pts));
}
