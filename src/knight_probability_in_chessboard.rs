const DIRECTION: [(isize, isize); 8] = [(-2,-1), (-1, -2), (-1, 2), (-2, 1), (1,-2), (2,-1), (2,1), (1,2)];

fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
    let (n, k, i, j) = (n as usize, k as usize, row as usize, column as usize);
    let mut dp = vec![vec![1.0; n]; n];

    for _ in 0..k {
        let mut temp_dp = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                for direct in DIRECTION {
                    if let (Some(ni), Some(nj)) = (i.checked_add_signed(direct.0), j.checked_add_signed(direct.1)) && ni < n && nj < n {
                        temp_dp[i][j] += 0.125 * dp[ni][nj];
                    }
                }
            }
        }

        dp = temp_dp;
    }

    dp[i][j]
}

pub fn main() {
    let n = 3;
    let k = 2;
    let row = 0;
    let column = 0;
    println!("{}", knight_probability(n, k, row, column));
}
