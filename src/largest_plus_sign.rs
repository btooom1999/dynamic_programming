fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut dp = vec![vec![[0; 4]; n+2]; n+2];
    let mut not_used = vec![vec![false; n]; n];
    for mine in mines {
        let (i, j) = (mine[0] as usize, mine[1] as usize);
        not_used[i][j] = true;
    }

    for i in 1..n+1 {
        for j in 1..n+1 {
            if !not_used[i-1][j-1] {
                dp[i][j][0] = dp[i][j-1][0] + 1;
            }
        }
    }

    for i in 1..n+1 {
        for j in 1..n+1 {
            if !not_used[i-1][j-1] {
                dp[i][j][1] = dp[i-1][j][1] + 1;
            }
        }
    }

    for i in 1..n+1 {
        for j in (1..n+1).rev() {
            if !not_used[i-1][j-1] {
                dp[i][j][2] = dp[i][j+1][2] + 1;
            }
        }
    }

    let mut max = 0;
    for i in (1..n+1).rev() {
        for j in 1..n+1 {
            if !not_used[i-1][j-1] {
                dp[i][j][3] = dp[i+1][j][3] + 1;
                max = max.max(*dp[i][j].iter().min().unwrap());
            }
        }
    }

    max
}

pub fn main() {
    let n = 5;
    let mines = [[3,0], [3,3]].into_iter().map(Vec::from).collect();
    println!("{}", order_of_largest_plus_sign(n, mines));
}

