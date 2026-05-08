pub fn main() {
    let n: usize = 4;
    let k: usize = 2;

    let mut dp: Vec<i32> = Vec::new();

    for i in 0..=k {
        dp.push((i+1) as i32);
    }

    for i in k+1..=n {
        dp.push(dp[i-1] + dp[i-k-1]);
    }

    println!("{}", dp[n]);
}
