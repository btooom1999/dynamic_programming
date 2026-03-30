fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut dp = vec![0; cost.len() + 2];
    for i in (0..cost.len()).rev() {
        dp[i] = cost[i] + std::cmp::min(dp[i+1], dp[i+2]);
    }

    dp[0].min(dp[1])
}

pub fn main() {
    let cost = [1,100,1,1,1,100,1,1,100,1];
    println!("{:?}", min_cost_climbing_stairs(cost.into()));
}
