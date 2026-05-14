fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut sum = nums[0];
    let mut res = nums[0];

    for i in 1..nums.len() {
        sum = nums[i].max(sum+nums[i]);
        res = res.max(sum);
    }

    res
}

pub fn main() {
    let nums = [-2,1,-3,4,-1,2,1,-5,4].to_vec();
    println!("{}", max_sub_array(nums));
}
