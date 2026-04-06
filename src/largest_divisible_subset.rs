use std::collections::HashMap;

fn dfs(
    idx: usize,
    nums: &Vec<i32>,
    num: i32,
    hashmap: &mut HashMap<(usize, i32), Vec<i32>>,
) -> Vec<i32> {
    if idx == nums.len() {
        return vec![];
    }

    if let Some(data) = hashmap.get(&(idx, num)) {
        return data.clone();
    }

    let mut res = Vec:: new();
    for i in idx..nums.len() {
        let mut temp = Vec::new();
        if nums[i] % num == 0 {
            temp.push(nums[i]);
            temp.extend(dfs(i+1, nums, nums[i], hashmap));
        }

        if temp.len() > res.len() {
            res = temp;
        }
    }

    hashmap.insert((idx, num), res.clone());
    res
}

fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort();

    dfs(0, &nums, 1, &mut HashMap::new())
}

pub fn main() {
    let nums = [1,2,3];
    println!("{:?}", largest_divisible_subset(nums.into()));
}
