fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (dungeon.len(), dungeon[0].len());
    let mut dp = vec![vec![i32::MIN; n]; m];
    dp[m-1][n-1] = dungeon[m-1][n-1];

    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if i+1<m {
                dp[i][j] = dp[i][j].max(dungeon[i][j] + dp[i+1][j]);
            }
            if j+1<n {
                dp[i][j] = dp[i][j].max(dungeon[i][j] + dp[i][j+1]);
            }

            dp[i][j] = dp[i][j].min(0);
        }
    }

    dp[0][0].abs()+1
}

pub fn main() {
    let dungeon = [[-2,-3,3],[-5,-10,1],[10,30,-5]].into_iter().map(Vec::from).collect();
    println!("{}", calculate_minimum_hp(dungeon));
}
