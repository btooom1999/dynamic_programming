fn jump(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = vec![-1; n];
    dp[n-1] = 0;

    for i in (0..n-1).rev() {
        let mut min = i32::MAX;
        for j in 1..=nums[i] as usize {
            let x = i+j;
            if x<n && j<=nums[i] as usize && dp[x] != i32::MAX {
                min = min.min(1 + dp[x]);
            }
        }

        dp[i] = min;
    }

    dp[0]
}

pub fn main() {
    let nums = [2,3,1,1,4].to_vec();
    println!("{}", jump(nums));
}
