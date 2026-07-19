fn backtracking(
    idx: usize,
    nums: &[i32],
    buckets: &mut [i32],
    target: i32,
) -> bool {
    if idx == nums.len() {
        return true;
    }

    let mut hashset = std::collections::HashSet::new();
    for i in 0..buckets.len() {
        let sum = buckets[i];
        if !hashset.insert(sum) || sum + nums[idx] > target {
            continue;
        }

        buckets[i] += nums[idx];
        if backtracking(idx+1, nums, buckets, target) { return true; }
        buckets[i] -= nums[idx];
    }

    false
}

fn can_partition_k_subsets(mut nums: Vec<i32>, k: i32) -> bool {
    let sum = nums.iter().sum::<i32>();
    if sum % k > 0 { return false; }

    let target = sum / k;

    nums.sort_by(|a, b| b.cmp(a));
    if nums[0] > target { return false; }

    let mut buckets = vec![0; k as usize];

    backtracking(0, &nums, &mut buckets, target)
}

pub fn main() {
    // let nums = [10,1,10,9,6,1,9,5,9,10,7,8,5,2,10,8].to_vec();
    // let k = 11;
    let nums = [1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4].to_vec();
    let k = 4;
    println!("{}", can_partition_k_subsets(nums, k));
}
