use std::collections::HashMap;

fn dfs(
    num: i64,
    memo: &mut HashMap<i64, i32>,
) -> i32 {
    if num == 1 {
        return 0;
    }

    if let Some(&step) = memo.get(&num) {
        return step;
    }

    let mut step = i32::MAX;
    if num % 2 == 1 {
        step = step.min(1+dfs(num-1, memo));
        step = step.min(1+dfs(num+1, memo));
    } else {
        step = 1+dfs(num/2, memo);
    }

    memo.insert(num, step);
    step
}

fn integer_replacement(n: i32) -> i32 {
    dfs(n as i64, &mut HashMap::new())
}

pub fn main() {
    let n = i32::MAX;
    println!("{}", integer_replacement(n));
}
