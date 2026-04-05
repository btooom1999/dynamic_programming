use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

fn knight_dialer(n: i32) -> i32 {
    let mut dp = [1_i64; 10];
    for _ in 1..n {
        dp = [
            (dp[4] + dp[6]) % MOD,
            (dp[6] + dp[8]) % MOD,
            (dp[7] + dp[9]) % MOD,
            (dp[4] + dp[8]) % MOD,
            (dp[0] + dp[3] + dp[9]) % MOD,
            0,
            (dp[0] + dp[1] + dp[7]) % MOD,
            (dp[2] + dp[6]) % MOD,
            (dp[1] + dp[3]) % MOD,
            (dp[2] + dp[4]) % MOD,
        ]
    }

    (dp.into_iter().sum::<i64>() % MOD) as i32
}

pub fn main() {
    let n = 3131;
    println!("{}", knight_dialer(n));
}
