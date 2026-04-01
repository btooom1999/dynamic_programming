fn dfs(
    n: usize,
    nums: &Vec<i32>,
    dp: &mut Vec<i32>,
) -> i32 {
    if n == 0 {
        return 0;
    }

    if dp[n] > 0 {
        return dp[n];
    }

    let mut res = 1e9 as i32;
    for &i in nums {
        if n as i32 - i < 0 {
            break;
        }

        res = res.min(1+dfs(n-i as usize, nums, dp));
    }

    dp[n] = res;
    res
}

fn num_squares(n: i32) -> i32 {
    let nums = (1..=10000).filter(|v| (*v as f32).sqrt() % 1_f32 == 0f32).collect::<Vec<_>>();
    let mut dp = vec![0; (n+1) as usize];
    dfs(n as usize, &nums, &mut dp)
}

pub fn main() {
    let n = 54;
    println!("{}", num_squares(n));
}
