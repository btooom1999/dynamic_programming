fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    let mut dp = vec![vec![0; n]; n];
    dp[0] = matrix[0].to_vec();

    for i in 1..n {
        for j in 0..n {
            dp[i][j] = matrix[i][j] + dp[i-1][j];

            if j > 0 {
                dp[i][j] = dp[i][j].min(dp[i-1][j-1]+matrix[i][j]);
            }
            if j+1 < n {
                dp[i][j] = dp[i][j].min(dp[i-1][j+1]+matrix[i][j]);
            }
        }
    }

    dp[n-1].clone().into_iter().min().unwrap_or(0)
}

pub fn main() {
    let matrix = [[2,1,3],[6,5,4],[7,8,9]].into_iter().map(Vec::from).collect();
    println!("{}", min_falling_path_sum(matrix));
}
