fn dfs(
    idx: usize,
    k: usize,
    partition: usize,
    nums: &Vec<i32>,
    memo: &mut Vec<Vec<f64>>,
) -> f64 {
    if memo[idx][partition] != -1f64 {
        return memo[idx][partition];
    }

    let mut res = 0_f64;
    let mut sum = 0_f64;
    for i in idx..nums.len() {
        if nums.len() - i - 1 < k-partition {
            break;
        }

        sum += nums[i] as f64;
        if partition < k {
            res = res.max(sum / (i-idx+1) as f64 + dfs(i+1, k, partition+1, nums, memo));
        } else {
            res = sum / (i-idx+1) as f64;
        }
    }

    memo[idx][partition] = res;
    res
}

fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
    let k = k as usize;
    dfs(0, k, 1, &nums, &mut vec![vec![-1f64; k+1]; nums.len()])
}

pub fn main() {
    // let nums = [1,2,3,4,5,6,7].to_vec();
    let nums = vec![10_000; 100];
    let k = 15;
    println!("{:?}", largest_sum_of_averages(nums, k));
}
