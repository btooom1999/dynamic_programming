fn max_absolute_sum(nums: Vec<i32>) -> i32 {
    let mut candidate = 0;
    let mut res = 0;
    for i in (0..nums.len()).rev() {
        candidate = nums[i].max(nums[i]+candidate);
        res = res.max(candidate);
    }

    candidate = 0;
    for i in (0..nums.len()).rev() {
        candidate = nums[i].min(nums[i]+candidate);
        res = res.max(candidate.abs());
    }

    res
}

pub fn main() {
    let nums = [2,-5,1,-4,3,-2].to_vec();
    println!("{}", max_absolute_sum(nums));
}
