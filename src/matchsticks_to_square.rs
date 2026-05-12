fn dfs(
    sum: i32,
    target: i32,
    need: i32,
    matchsticks: &Vec<i32>,
    visited: &mut i32,
    dp: &mut Vec<Vec<bool>>,
) -> bool {
    if need == 0 {
        return true;
    }

    let i = *visited as usize;
    let j = need as usize;
    if dp[i][j] {
        return false;
    }

    for i in 0..matchsticks.len() {
        let value = sum + matchsticks[i];
        if *visited & (1 << i) == 0 || value > target {
            continue;
        }

        *visited ^= 1 << i;
        let is_valid = value == target;
        let need = need - is_valid as i32;
        let value = if is_valid { 0 } else { value };
        if dfs(value, target, need, matchsticks, visited, dp) {
            return true;
        }
        *visited ^= 1 << i;
    }

    dp[i][j] = true;
    false
}

fn makesquare(matchsticks: Vec<i32>) -> bool {
    let n = matchsticks.len();
    if n < 4 {
        return false;
    }

    let sum = matchsticks.iter().sum::<i32>();
    if sum % 4 != 0 {
        return false;
    }

    let n = 1 << (n+1);
    dfs(0, sum/4, 4, &matchsticks, &mut (n-1), &mut vec![vec![false; 5]; n as usize])
}

pub fn main() {
    let matchsticks = [1,1,6,6,8].to_vec();
    println!("{}", makesquare(matchsticks));
}
