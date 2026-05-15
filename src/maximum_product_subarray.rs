fn max_product(nums: Vec<i32>) -> i32 {
    let mut min = nums[0];
    let mut max = min;
    let mut res = min;

    for i in 1..nums.len() {
        let temp = [nums[i], nums[i]*max, nums[i]*min];
        min = *temp.iter().min().unwrap();
        max = *temp.iter().max().unwrap();
        res = res.max(max);
    }

    res
}

pub fn main() {
    let nums = [2,3,-2,-4].to_vec();
    println!("{}", max_product(nums));
}
