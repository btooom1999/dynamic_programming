fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut dp = vec![vec![i32::MAX; n]; m];

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if i == m-1 && j == n-1 {
                dp[i][j] = grid[i][j];
            }
            if i+1 != m {
                dp[i][j] = grid[i][j] + dp[i+1][j];
            }
            if j+1 != n {
                dp[i][j] = dp[i][j].min(grid[i][j]+dp[i][j+1]);
            }
        }
    }

    dp[0][0]
}

pub fn main() {
    let grid = [[1,3,1],[1,5,1],[4,2,1]].into_iter().map(Vec::from).collect();
    println!("{}", min_path_sum(grid));
}
