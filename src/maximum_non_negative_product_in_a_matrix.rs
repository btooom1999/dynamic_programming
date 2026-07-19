const MOD: i64 = 1_000_000_007;

fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut dp = vec![vec![(i64::MIN, i64::MAX); n]; m];

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            let val = grid[i][j] as i64;
            if i+1<m {
                dp[i][j].0 = dp[i][j].0.max(val * dp[i+1][j].0).max(val * dp[i+1][j].1);
                dp[i][j].1 = dp[i][j].1.min(val * dp[i+1][j].0).min(val * dp[i+1][j].1);
            }
            if j+1<n {
                dp[i][j].0 = dp[i][j].0.max(val * dp[i][j+1].0).max(val * dp[i][j+1].1);
                dp[i][j].1 = dp[i][j].1.min(val * dp[i][j+1].0).min(val * dp[i][j+1].1);
            }
            if i+1==m && j+1==n {
                dp[i][j] = (val, val);
            }
        }
    }

    if dp[0][0].0 < 0 { -1 } else { (dp[0][0].0 % MOD) as i32 }
}

pub fn main() {
    // let grid = [[1,-2,1],[1,-2,1],[3,-4,1]].into_iter().map(Vec::from).collect();
    let grid = [[2,1,3,0,-3,3,-4,4,0,-4],[-4,-3,2,2,3,-3,1,-1,1,-2],[-2,0,-4,2,4,-3,-4,-1,3,4],[-1,0,1,0,-3,3,-2,-3,1,0],[0,-1,-2,0,-3,-4,0,3,-2,-2],[-4,-2,0,-1,0,-3,0,4,0,-3],[-3,-4,2,1,0,-4,2,-4,-1,-3],[3,-2,0,-4,1,0,1,-3,-1,-1],[3,-4,0,2,0,-2,2,-4,-2,4],[0,4,0,-3,-4,3,3,-1,-2,-2]].into_iter().map(Vec::from).collect();
    println!("{}", max_product_path(grid));
}
