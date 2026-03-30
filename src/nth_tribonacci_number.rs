fn tribonacci(n: i32) -> i32 {
    let n = n as usize;
    if n <= 2 {
        return (n != 0) as i32;
    }

    let mut sum = 0;
    let mut dp = vec![0; n+1];
    dp[1] = 1;
    dp[2] = 1;

    for i in 3..=n {
        dp[i] = dp[i-1] + dp[i-2] + dp[i-3];
    }

    dp[n]
}

pub fn main() {
    let n = 2;
    println!("{}", tribonacci(n));
}
