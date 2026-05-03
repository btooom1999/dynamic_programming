fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut dp = vec![vec![0; n+1]; m+1];
    let mut max = 0;

    for i in 1..m+1 {
        for j in 1..n+1 {
            if matrix[i-1][j-1] == '1' {
                dp[i][j] = dp[i-1][j].min(dp[i-1][j-1]).min(dp[i][j-1]) + 1;
                max = max.max(dp[i][j]);
            }
        }
    }

    max * max
}

pub fn main() {
    let matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
        .into_iter()
        .map(|sub| sub.into_iter().map(|c| c.chars().next().unwrap()).collect())
        .collect();
    println!("{}", maximal_square(matrix));
}
