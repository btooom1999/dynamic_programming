fn max_profit(prices: Vec<i32>) -> i32 {
    let n = prices.len();
    let mut dp = vec![0; n+2];
    let mut res = 0;
    for i in (0..n-1).rev() {
        dp[i] = dp[i+1];
        for j in i+1..n {
            dp[i] = dp[i].max(prices[j]-prices[i]+dp[j+2]);
            res = res.max(dp[i]);
        }
    }

    res
}

pub fn main() {
    let prices = [1,2,3,0,2].to_vec();
    println!("{}", max_profit(prices));
}
