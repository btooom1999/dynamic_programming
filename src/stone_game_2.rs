fn dfs(
    alice: usize,
    i: usize,
    m: usize,
    piles: &Vec<i32>,
    dp: &mut Vec<Vec<Vec<i32>>>,
) -> i32 {
    if i == piles.len() {
        return 0;
    }

    if dp[alice][i][m] != -1 {
        return dp[alice][i][m];
    }

    let mut res = if alice == 1 { 0 } else { i32::MAX };
    let mut total = 0;

    for x in 1..2*m+1 {
        if i+x > piles.len() {
            break;
        }

        total += piles[i+x-1];
        if alice == 1 {
            res = res.max(total + dfs(0, i+x, m.max(x), piles, dp));
        } else {
            res = res.min(dfs(1, i+x, m.max(x), piles, dp));
        }
    }

    dp[alice][i][m] = res;
    res
}

fn stone_game_ii(piles: Vec<i32>) -> i32 {
    dfs(1, 0, 1, &piles, &mut vec![vec![vec![-1; piles.len()*2]; piles.len()]; 2])
}

pub fn main() {
    // let piles = [2,7,9,4,4].to_vec();
    let piles = vec![1; 1];
    println!("{}", stone_game_ii(piles));
}
