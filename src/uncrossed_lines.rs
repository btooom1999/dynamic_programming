use std::collections::HashMap;

fn dfs(
    idx: usize,
    limit: usize,
    hashmap: &mut HashMap<(usize, usize), i32>,
    nums1: &Vec<i32>,
    nums2: &Vec<i32>,
) -> i32 {
    if idx >= nums1.len() {
        return 0;
    }

    if let Some(&max) = hashmap.get(&(idx, limit)) {
        return max;
    }

    let mut res = dfs(idx+1, limit, hashmap, nums1, nums2);
    for i in limit..nums2.len() {
        if nums2[i] != nums1[idx] {
            continue;
        }

        res = res.max(1 + dfs(idx+1, i+1, hashmap, nums1, nums2));
    }

    hashmap.insert((idx, limit), res);
    res
}

fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    dfs(0, 0, &mut HashMap::new(), &nums1, &nums2)
}

pub fn main() {
    let nums1 = [1,4,2];
    let nums2 = [1,2,4];
    println!("{}", max_uncrossed_lines(nums1.into(), nums2.into()));
}
