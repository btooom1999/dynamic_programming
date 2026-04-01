fn valid_partition(nums: Vec<i32>) -> bool {
    let mut dp = vec![false; nums.len()+1];
    dp[nums.len()] = true;

    for i in (0..nums.len()).rev() {
        if i+1 < nums.len() && nums[i] == nums[i+1] && dp[i+2] {
            dp[i] = dp[i+2];
        }

        if !dp[i] && i+1 < nums.len() && i+2 < nums.len() && dp[i+3] {
            if nums[i] == nums[i+1] && nums[i] == nums[i+2] {
                dp[i] = dp[i+3];
            }

            if !dp[i] && nums[i]+1 == nums[i+1] && nums[i+1]+1 == nums[i+2] {
                dp[i] = dp[i+3];
            }
        }
    }

    dp[0]
}

pub fn main() {
    let nums = vec![1,1,1,2];
    println!("{}", valid_partition(nums));
}
