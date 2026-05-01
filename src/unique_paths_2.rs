fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
    if obstacle_grid[0][0] == 1 || obstacle_grid[m-1][n-1] == 1 {
        return 0;
    }

    let mut dp = vec![vec![0; n]; m];
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if obstacle_grid[i][j] == 1 {
                continue;
            }
            if i == m-1 && j == n-1 {
                dp[i][j] = 1;
            }
            if i+1 != m {
                dp[i][j] += dp[i+1][j];
            }
            if j+1 != n {
                dp[i][j] += dp[i][j+1];
            }
        }
    }

    dp[0][0]
}

pub fn main() {
    let obstacle_grid = [[0,0,0],[0,1,0],[0,0,0]].into_iter().map(Vec::from).collect();
    println!("{:?}", unique_paths_with_obstacles(obstacle_grid));
}
