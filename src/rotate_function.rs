fn max_rotate_function(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut sum = 0;
    let mut f = 0;
    for (mul, &num) in nums.iter().enumerate() {
        sum += num;
        f += num * mul as i32;
    }

    let mut res = f;
    for i in 1..n {
        f = f + sum - nums[n-i] * n as i32;
        res = res.max(f);
    }

    res
}

pub fn main() {
    let nums = [4,3,2,6].to_vec();
    println!("{}", max_rotate_function(nums));
}
