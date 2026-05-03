fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut dp = vec![vec![0; n+1]; m+1];

    let mut count = 0;
    for i in 1..m+1 {
        for j in 1..n+1 {
            if matrix[i-1][j-1] == 1 {
                dp[i][j] = dp[i-1][j].min(dp[i-1][j-1]).min(dp[i][j-1]) + 1;
                count += dp[i][j];
            }
        }
    }

    count
}

pub fn main() {
    let matrix = [[0,1,1,1], [1,1,1,1], [0,1,1,1]].into_iter().map(Vec::from).collect();
    println!("{:?}", count_squares(matrix));
}
