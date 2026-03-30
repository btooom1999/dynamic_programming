fn rob(nums: Vec<i32>) -> i32 {
    let mut dp = vec![0; nums.len() + 3];
    for i in (0..nums.len()).rev() {
        dp[i] = nums[i] + dp[i+2].max(dp[i+3]);
    }

    dp[0].max(dp[1])
}

pub fn main() {
    let nums = [2,7,9,3,1];
    println!("{}", rob(nums.into()));
}
