fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut dp = vec![vec![[0;4]; n+2]; m+2];

    for i in 1..m+1 {
        for j in 1..n+1 {
            if grid[i-1][j-1] == 1 {
                dp[i][j][0] = dp[i][j-1][0] + 1;
            }
        }
    }

    for i in 1..m+1 {
        for j in 1..n+1 {
            if grid[i-1][j-1] == 1 {
                dp[i][j][1] = dp[i-1][j][1] + 1;
            }
        }
    }

    for i in 1..m+1 {
        for j in (1..n+1).rev() {
            if grid[i-1][j-1] == 1 {
                dp[i][j][2] = dp[i][j+1][2] + 1;
            }
        }
    }

    for i in (1..m+1).rev() {
        for j in 1..n+1 {
            if grid[i-1][j-1] == 1 {
                dp[i][j][3] = dp[i+1][j][3] + 1;
            }
        }
    }

    let mut res = 0;
    let mut max = 0;
    for i in 1..m+1 {
        for j in 1..n+1 {
            for delta in max..=dp[i][j][0].min(dp[i][j][1]) {
                let origin = dp[i+1-delta][j+1-delta];
                if delta <= origin[2].min(origin[3]) {
                    res = res.max(delta);
                }
            }

            max = res;
        }
    }

    (res*res) as i32
}

pub fn main() {
    // let grid = [[1,1,1,1],[1,0,0,1],[1,0,0,1],[1,1,1,1]].into_iter().map(Vec::from).collect();
    let grid = [[0,1,1,1,1,0],[1,1,0,1,1,0],[1,1,0,1,0,1],[1,1,0,1,1,1],[1,1,0,1,1,1],[1,1,1,1,1,1],[1,0,1,1,1,1],[0,0,1,1,1,1],[1,1,1,1,1,1]].into_iter().map(Vec::from).collect();
    println!("{}", largest1_bordered_square(grid));
}
