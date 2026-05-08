fn minimum_edge_cover(k: Vec<i32>, n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![i32::MAX; n];
    dp[1] = k[0];

    for i in 2..n {
        dp[i] = std::cmp::min(dp[i-2], dp[i-1]) + k[i-1];
    }

    dp[n-1]
}

pub fn main() {
    let n = 6;
    let k = [2, 2, 3, 2, 2].to_vec();
    println!("{}", minimum_edge_cover(k, n));
}
