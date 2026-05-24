fn dfs(
    is_alice: bool,
    i: usize,
    j: usize,
    nums: &Vec<i32>,
    memo: &mut Vec<Vec<Vec<i32>>>,
) -> i32 {
    if i > j {
        return 0;
    }

    if i == j {
        return if is_alice { nums[i] } else { 0 };
    }

    if memo[i][j][is_alice as usize] > -1 {
        return memo[i][j][is_alice as usize];
    }

    let a = if is_alice { nums[i] } else { 0 } + dfs(!is_alice, i+1, j, nums, memo);
    let b = if is_alice { nums[j] } else { 0 } + dfs(!is_alice, i, j-1, nums, memo);
    let point = if is_alice {
        a.max(b)
    } else {
        a.min(b)
    };


    memo[i][j][is_alice as usize] = point;
    point
}

fn predict_the_winner(nums: Vec<i32>) -> bool {
    let sum = nums.iter().sum::<i32>();
    let n = nums.len();
    let alice = dfs(true, 0, n-1, &nums, &mut vec![vec![vec![-1; 2]; n]; n]);
    alice >= (sum - alice)
}

pub fn main() {
    let nums = [1,3,1].to_vec();
    // let nums = [1,5,233,7].to_vec();
    println!("{}", predict_the_winner(nums));
}
