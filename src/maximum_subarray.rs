fn max_sub_array(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut max = 0;
    let mut res = i32::MIN;
    for i in (0..n).rev() {
        max = nums[i].max(nums[i]+max);
        res = res.max(max);
    }

    res
}

pub fn main() {
    let nums = [-2,1,-3,4,-1,2,1,-5,4].to_vec();
    println!("{}", max_sub_array(nums));
}
