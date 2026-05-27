const MOD: i32 = 1_000_000_007;

fn num_tilings(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![vec![0; n+1]; n+1];
    dp[0][0] = 1;

    for i in 0..n+1 {
        for j in 0..n+1 {
            if dp[i][j] > 0 {
                let tiles = if j > i {
                    vec![(2,0), (2,1)]
                } else if i > j {
                    vec![(0,2), (1,2)]
                } else {
                    vec![(1,1), (2,2), (2,1), (1,2)]
                };

                for tile in tiles {
                    let ni = i + tile.0;
                    let nj = j + tile.1;
                    if ni <= n && nj <= n {
                        dp[ni][nj] = (dp[ni][nj] + dp[i][j]) % MOD;
                    }
                }
            }
        }
    }

    dp[n][n]
}

pub fn main() {
    let n = 1000;
    println!("{}", num_tilings(n));
}
