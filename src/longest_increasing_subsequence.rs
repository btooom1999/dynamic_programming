use std::collections::HashMap;

fn dfs(
    index: usize,
    nums: &Vec<i32>,
    hashmap: &mut HashMap<i32, i32>,
    use_hashmap: bool,
) -> i32 {
    if use_hashmap && let Some(&max) = hashmap.get(&nums[index]) {
        return max;
    }

    let mut res = 1;
    for i in index..nums.len() {
        if nums[index] < nums[i] {
            res = res.max(1 + dfs(i, nums, hashmap, true));
        }
    }

    hashmap.insert(nums[index], res);
    res
}

fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut hashmap = HashMap::new();
    let mut res = 0;
    for i in (0..nums.len()).rev() {
        res = res.max(dfs(i, &nums, &mut hashmap, false));
    }

    res
}

pub fn main() {
    let nums = [1,3,6,7,9,4,10,5,6];
    println!("{}", length_of_lis(nums.into()));
}
