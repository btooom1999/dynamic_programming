fn change(amount: i32, coins: Vec<i32>) -> i32 {
    let m = amount as usize;
    let n = coins.len();
    let mut dp = vec![0; m+1];
    dp[0] = 1;

    for i in 1..=n {
        let x = coins[i-1] as usize;
        for j in x..=m {
            dp[j] += dp[j-x];
        }
    }

    dp[m]
}

pub fn main() {
    // let amount = 5;
    // let coins = [1,2,5].to_vec();
    let amount = 100;
    let coins = (1..=300).collect();
    println!("{}", change(amount, coins));
}
