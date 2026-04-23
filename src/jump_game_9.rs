fn max_value(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut max = vec![0; n];
    max[0] = nums[0];
    for i in 1..n {
        if nums[i] < max[i-1] {
            max[i] = max[i-1];
        } else {
            max[i] = nums[i];
        }
    }

    let mut min = vec![0; n];
    min[n-1] = nums[n-1];
    for i in (0..n-1).rev() {
        if min[i+1] < nums[i] {
            min[i] = min[i+1];
        } else {
            min[i] = nums[i];
        }
    }

    let mut res = vec![-1; n];
    res[n-1] = max[n-1];
    for i in (0..n-1).rev() {
        res[i] = max[i];
        if max[i] > min[i+1] {
            res[i] = res[i+1];
        }
    }

    res
}

pub fn main() {
    let nums = [20,21,25,15].to_vec();
    println!("{:?}", max_value(nums));
}
