use std::collections::HashSet;

fn dfs(
    i: usize,
    nums: &Vec<i32>,
    stack: Vec<(usize, usize)>,
    parentheses: usize,
    memo: &mut HashSet<Vec<(usize, usize)>>,
    max: &mut (String, f64),
) {
    memo.insert(stack.clone());
    if i == nums.len() {
        let mut total = 0f64;
        for i in 0..stack.len() {
            let mut res = nums[stack[i].0] as f64;
            for i in stack[i].0+1..stack[i].1+1 {
                res /= nums[i] as f64;
            }
            if i == 0 {
                total = res;
            } else {
                total /= res;
            }
        }
        if total > max.1 {
            *max = (stack.into_iter().fold(String::new(), |mut acc, (i, j)| {
                if i == j {
                    acc.push_str(&nums[i].to_string());
                    if i+1 != nums.len() {
                        acc.push('/');
                    }
                } else {
                    acc.push('(');
                    for x in i..=j {
                        acc.push_str(&nums[x].to_string());
                        if x != j {
                            acc.push('/');
                        }
                    }
                    acc.push(')');
                }

                acc
            }), total);
        }
    } else {
        let mut another_stack = stack.clone();
        another_stack.push((i, i));
        dfs(i+1, nums, another_stack.clone(), parentheses+1, memo, max);

        for j in 1..=parentheses {
            let mut another_stack = stack.clone();
            for _ in 0..j {
                let mut value = another_stack.pop().unwrap();
                value.1 = i;
                another_stack.push(value);
                if !memo.contains(&another_stack) {
                    dfs(i+1, nums, another_stack.clone(), parentheses-j, memo, max);
                }
                another_stack.pop();
            }
        }
    }
}

fn optimal_division(nums: Vec<i32>) -> String {
    let mut max = (String::new(), 0f64);
    dfs(0, &nums, Vec::new(), 0, &mut HashSet::new(), &mut max);

    max.0
}

pub fn main() {
    let nums = [1000, 100, 10, 2].to_vec();
    println!("{}", optimal_division(nums));
}
