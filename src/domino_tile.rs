pub fn main() {
    let n = 5;
    let mut dp = [0; 100];

    dp[1] = 1; // vertical tile 2x1
    dp[2] = 2; // horizontal tile 2x2

    for i in 3..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    println!("{}", dp[n]);
}
