fn dfs(
    k: usize,
    target: i32,
    sum: i32,
    visited: &mut usize,
    nums: &Vec<i32>,
    memo: &mut Vec<Vec<bool>>,
) -> bool {
    if k == 0 {
        return *visited == 0;
    }

    if sum == target {
        return dfs(k-1, target, 0, visited, nums, memo);
    }

    if memo[*visited][k] {
        return false;
    }

    for i in 0..nums.len() {
        if sum + nums[i] > target {
            break;
        }
        if *visited & (1 << i) == 0 {
            continue;
        }

        *visited ^= 1 << i;
        if dfs(k, target, sum+nums[i], visited, nums, memo) {
            return true;
        }
        *visited ^= 1 << i;
    }


    memo[*visited][k] = true;
    false
}

fn can_partition_k_subsets(mut nums: Vec<i32>, k: i32) -> bool {
    let n = nums.len();
    if n < k as usize {
        return false;
    }

    let sum = nums.iter().sum::<i32>();
    if sum % k != 0 {
        return false;
    }

    let n = (1 << n) as usize;
    let target = sum / k;
    let k = k as usize;
    nums.sort();
    dfs(k, target, 0, &mut (n-1), &nums, &mut vec![vec![false; k+1]; n])
}

pub fn main() {
    // let nums = [10,1,10,9,6,1,9,5,9,10,7,8,5,2,10,8].to_vec();
    // let k = 11;
    let nums = [1,1,1,1,2,2,2,2,3,3,3,3,4,4,4,4].to_vec();
    let k = 4;
    println!("{}", can_partition_k_subsets(nums, k));
}
