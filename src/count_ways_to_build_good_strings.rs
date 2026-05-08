const MOD: i32 = 1_000_000_007;

fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
    let low = low as usize;
    let high = high as usize;
    let zero = zero as usize;
    let one = one as usize;

    let mut dp = vec![0; high+1];
    dp[0] = 1;

    for i in 1..=high {
        if i >= zero {
            dp[i] = (dp[i] + dp[i-zero]) % MOD;
        }

        if i >= one {
            dp[i] = (dp[i] + dp[i-one]) % MOD;
        }
    }

    let mut count = 0;
    for i in low..=high {
        count = (count + dp[i]) % MOD;
    }

    count
}

pub fn main() {
    let low = 200;
    let high = 200;
    let zero = 10;
    let one = 1;
    println!("{}", count_good_strings(low, high, zero, one));
}
