fn num_teams(rating: Vec<i32>) -> i32 {
    let n = rating.len();
    let mut dp = vec![(0,0); n];

    let mut res = 0;
    for i in (0..n-1).rev() {
        for j in i+1..n {
            if rating[i] == rating[j] {
                continue;
            } else if rating[i] > rating[j] {
                res += dp[j].1;
                dp[i].1 += 1;
            } else {
                res += dp[j].0;
                dp[i].0 += 1;
            }
        }
    }

    res
}

pub fn main() {
    let rating = [2,5,3,4,1].to_vec();
    println!("{}", num_teams(rating));
}
