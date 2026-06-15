fn get_money_amount(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![vec![0; n+1]; n+1];
    for dist in 2..=n {
        println!();
        for i in 1..=n-dist+1 {
            let j = i+dist-1;
            let mut res = i32::MAX;
            for guess in i..=j {
                let worst = guess as i32 + dp[i][guess.max(1)-1].max(dp[(guess+1).min(j)][j]);
                res = res.min(worst);
            }

            dp[i][j] = res;
        }
    }

    dp[1][n]
}

pub fn main() {
    let n = 10;
    println!("{}", get_money_amount(n));
}
