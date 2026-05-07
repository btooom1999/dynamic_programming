use std::collections::HashMap;

fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let mut dp = HashMap::from([
        (nums[0], 1),
    ]);

    if nums[0] == 0 {
        dp.insert(nums[0], 2);
    } else {
        dp.insert(-nums[0], 1);
    }

    for i in 1..nums.len() {
        let mut new_dp = HashMap::<i32, i32>::new();
        for num in [nums[i], -nums[i]] {
            for (&key, &value) in &dp {
                *new_dp.entry(key+num).or_default() += value;
            }
        }

        dp = new_dp;
    }

    dp.remove(&target).unwrap_or(0)
}

pub fn main() {
    let nums = vec![1; 20];
    let target = 20;
    println!("{}", find_target_sum_ways(nums, target));
}
