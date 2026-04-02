fn dfs(
    dp: &mut Vec<i32>,
    n: usize,
    max: usize,
) -> i32 {
    if dp[n] > 0 {
        return dp[n];
    }

    dp[n] = if n == max { 0 } else { n as i32 };
    for i in 1..n {
        let val = dfs(dp, i, max) * dfs(dp, n-i, max);
        dp[n] = dp[n].max(val);
    }

    dp[n]
}

fn integer_break(n: i32) -> i32 {
    let mut dp = vec![0; n as usize+1];
    dp[1] = 1;
    dfs(&mut dp, n as usize, n as usize);

    dp[n as usize]
}

pub fn main() {
    let n = 5;
    println!("{}", integer_break(n));
}
