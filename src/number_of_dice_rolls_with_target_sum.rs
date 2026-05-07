const MOD: i32 = 1_000_000_007;

fn dfs(
    i: usize,
    n: usize,
    k: i32,
    target: i32,
    dp: &mut Vec<Vec<i32>>,
) -> i32 {
    if target < 0 || i > n {
        return (target == 0) as i32;
    }

    let target = target as usize;
    if dp[i][target] == -1 {
        for num in 1..k+1 {
            dp[i][target] = (dp[i][target].max(0) + dfs(i+1, n, k, target as i32 - num, dp)) % MOD;
        }
    }

    dp[i][target]
}

fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
    dfs(1, n as usize, k, target, &mut vec![vec![-1; (target+1) as usize]; (n+1) as usize])
}

pub fn main() {
    let n = 30;
    let k = 30;
    let target = 500;
    println!("{}", num_rolls_to_target(n, k, target));
}

