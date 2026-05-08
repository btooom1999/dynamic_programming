fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
    let last_day = days[days.len()-1] as usize;
    let mut exist_days = vec![false; last_day + 2];
    for &day in &days {
        exist_days[day as usize] = true;
    }

    let mut dp = vec![0; last_day+31];
    for i in (0..=last_day).rev() {
        if !exist_days[i] {
            dp[i] = dp[i+1];
            continue;
        }

        dp[i] = costs[0] + dp[i+1];
        dp[i] = dp[i].min(costs[1] + dp[i+7]);
        dp[i] = dp[i].min(costs[2] + dp[i+30]);
    }

    dp[0]
}

pub fn main() {
    let days = [1,4,6,7,8,20];
    let costs = [2,7,15];
    println!("{}", mincost_tickets(days.into(), costs.into()));
}
