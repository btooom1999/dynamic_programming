use std::collections::HashMap;

fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = vec![HashMap::new(); n];
    dp[n-1].insert(nums[n-1], 1);
    for i in (0..n-1).rev() {
        for j in i+1..n {
            for (key, value) in dp[j].clone() {
                *dp[i].entry(nums[i] | key).or_default() += value;
            }
        }
        *dp[i].entry(nums[i]).or_default() += 1;
    }

    *dp
        .into_iter()
        .fold(HashMap::new(), |mut acc, map| {
            for (k, v) in map {
                *acc.entry(k).or_default() += v;
            }

            acc
        })
        .values()
        .max()
        .unwrap_or(&0)
}

pub fn main() {
    let nums = [3,2,1,5].to_vec();
    println!("{}", count_max_or_subsets(nums));
}
