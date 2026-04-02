fn dfs(
    idx: usize,
    n: usize,
    nums: &Vec<i32>,
    dp: &mut Vec<(i32, i32)>,
    max_len: &mut i32,
) -> (i32, i32) {
    if dp[idx].0 > 0 {
        return dp[idx];
    }

    let mut count = (1,1);
    for i in idx..n {
        if nums[idx] < nums[i] {
            let max = dfs(i, n, nums, dp, max_len);

            if max.0+1 >= count.0 {
                if max.0+1 > count.0 {
                    count = (max.0+1, 0);
                }
                count.1 += max.1;
            }

        }
    }

    *max_len = std::cmp::max(*max_len, count.0);

    dp[idx] = count;
    count
}

fn find_number_of_lis(nums: Vec<i32>) -> i32 {
    let mut max_len = 1;
    let mut dp = vec![(0,0); nums.len()];
    dp[nums.len()-1] = (1,1);
    for i in (0..nums.len()).rev() {
        dfs(i, nums.len(), &nums, &mut dp, &mut max_len);
    }

    let mut res = 0;
    for i in 0..nums.len() {
        if max_len == dp[i].0 {
            res += dp[i].1;
        }
    }

    res
}

pub fn main() {
    let nums = [2,2,2,2,2];
    println!("{}", find_number_of_lis(nums.into()));
}
