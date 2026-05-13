fn dfs(
    i: usize,
    matchsticks: &Vec<i32>,
    side: i32,
    sides: &mut [i32; 4],
) -> bool {
    if i == matchsticks.len() {
        return sides.iter().all(|&s| s == side);
    }

    for j in 0..4 {
        if sides[j] + matchsticks[i] <= side {
            sides[j] += matchsticks[i];
            if dfs(i+1, matchsticks, side, sides) {
                return true;
            }
            sides[j] -= matchsticks[i];
        }
    }

    false
}

fn makesquare(mut matchsticks: Vec<i32>) -> bool {
    let n = matchsticks.len();
    if n < 4 {
        return false;
    }

    let sum = matchsticks.iter().sum::<i32>();
    if sum % 4 != 0 {
        return false;
    }

    matchsticks.sort_by(|a, b| b.cmp(a));
    dfs(0, &matchsticks, sum/4, &mut [0;4])
}

pub fn main() {
    let matchsticks = [1,1,1,1,1,1,1,1,1,1,1,1,1,1,6].to_vec();
    println!("{}", makesquare(matchsticks));
}
