fn most_points(questions: Vec<Vec<i32>>) -> i64 {
    let mut dp = vec![0; questions.len()+1];
    let mut res = 0;
    for i in (0..questions.len()).rev() {
        dp[i] = questions[i][0] as i64;
        if i +1+(questions[i][1] as usize) < questions.len() {
            dp[i] += dp[i+1+questions[i][1] as usize] as i64;
        }

        dp[i] = dp[i].max(dp[i+1]);

        res = res.max(dp[i]);
    }

    res
}

pub fn main() {
    let questions = [[3,2],[4,3],[4,4],[2,5]].map(|v| v.to_vec()).into_iter().collect();
    println!("{}", most_points(questions));
}
