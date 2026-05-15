fn max_absolute_sum(nums: Vec<i32>) -> i32 {
    let mut min = nums[0];
    let mut max = min;
    let mut res = min.abs();

    for i in 1..nums.len() {
        let temp = [nums[i], nums[i]+min, nums[i]+max];
        min = *temp.iter().min().unwrap();
        max = *temp.iter().max().unwrap();
        res = res.max(min.abs()).max(max.abs());
    }

    res
}

pub fn main() {
    let nums = [2,-5,1,-4,3,-2].to_vec();
    println!("{}", max_absolute_sum(nums));
}
