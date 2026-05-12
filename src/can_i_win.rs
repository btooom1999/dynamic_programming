fn dfs(
    max_choosable_integer: i32,
    desired_total: i32,
    visited: &mut i32,
    dp: &mut Vec<i32>,
) -> bool {
    if desired_total <= 0 {
        return false;
    }

    let k = *visited as usize;
    if dp[k] != -1 {
        return dp[k] == 1;
    }

    let mut win = false;
    for i in 1..max_choosable_integer + 1 {
        if *visited & (1 << i) == 0 {
            continue;
        }

        *visited ^= 1 << i;
        win = win || !dfs(max_choosable_integer, desired_total-i, visited, dp);
        *visited ^= 1 << i;
    }


    dp[k] = win as i32;
    win
}

fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
    if max_choosable_integer >= desired_total {
        return true;
    }

    if max_choosable_integer * (max_choosable_integer+1) / 2 < desired_total {
        return false;
    }

    let n = 1 << (max_choosable_integer+1);
    dfs(max_choosable_integer, desired_total, &mut (n-1), &mut vec![-1; n as usize])
}

pub fn main() {
    // let max_choosable_integer = 4;
    // let desired_total = 6;
    let max_choosable_integer = 10;
    let desired_total = 100;
    println!("{}", can_i_win(max_choosable_integer, desired_total));
}
