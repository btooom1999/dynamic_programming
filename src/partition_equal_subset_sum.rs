use std::collections::HashSet;

fn dfs(
    idx: usize,
    target: i32,
    nums: &Vec<i32>,
    used: &mut Vec<i32>,
    memo: &mut HashSet<(usize, i32)>,
) -> bool {
    if target < 0 || memo.get(&(idx, target)).is_some() {
        return false;
    }

    if target == 0 {
        return true;
    }

    for i in idx..nums.len() {
        used[nums[i] as usize] += 1;
        if dfs(i+1, target-nums[i], nums, used, memo) {
            return true;
        } else {
            memo.insert((i, target));
        }
        used[nums[i] as usize] -= 1;
    }

    false
}

fn can_partition(nums: Vec<i32>) -> bool {
    let sum = nums.iter().sum::<i32>();
    if sum % 2 == 1 {
        return false;
    }

    dfs(0, sum / 2, &nums, &mut vec![0; 101], &mut HashSet::new())
}

pub fn main() {
    let nums = [4,4,4,4,4,4,4,4,8,8,8,8,8,8,8,8,12,12,12,12,12,12,12,12,16,16,16,16,16,16,16,16,20,20,20,20,20,20,20,20,24,24,24,24,24,24,24,24,28,28,28,28,28,28,28,28,32,32,32,32,32,32,32,32,36,36,36,36,36,36,36,36,40,40,40,40,40,40,40,40,44,44,44,44,44,44,44,44,48,48,48,48,48,48,48,48,52,52,52,52,52,52,52,52,56,56,56,56,56,56,56,56,60,60,60,60,60,60,60,60,64,64,64,64,64,64,64,64,68,68,68,68,68,68,68,68,72,72,72,72,72,72,72,72,76,76,76,76,76,76,76,76,80,80,80,80,80,80,80,80,84,84,84,84,84,84,84,84,88,88,88,88,88,88,88,88,92,92,92,92,92,92,92,92,96,96,96,96,96,96,96,96,97,99];
    println!("{}", can_partition(nums.into()));
}
