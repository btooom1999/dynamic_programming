pub fn main() {
    let n = 4;
    let broken_steps = [2];

    let mut dp = vec![0; n+1];
    dp[1] = 1;

    // 0 is secure steps
    // 1 is insecure steps
    let mut status_of_steps = vec![0;n+1];
    for i in broken_steps.iter() {
        status_of_steps[i.to_owned()] = 1;
    }

    for i in 2..=n {
        if status_of_steps[i] == 1 { continue; }
        dp[i] = dp[i-1] + dp[i-2];
    }

    println!("{}", dp[n]);
}
