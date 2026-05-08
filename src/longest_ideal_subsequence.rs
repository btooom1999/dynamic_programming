use std::collections::HashMap;

fn longest_ideal_string(s: String, k: i32) -> i32 {
    let n = s.len();
    let s = s.as_bytes();
    let mut hashmap = HashMap::new();
    let mut dp = vec![0; n];
    dp[n-1] = 1;
    hashmap.insert((s[n-1]-b'a') as i32, 1);

    let mut res = dp[n-1];
    for i in (0..n-1).rev() {
        let num = (s[i]-b'a') as i32;
        let mut maximum = 0;
        for i in num-k..num+k+1 {
            if let Some(&max) = hashmap.get(&i) {
                maximum = maximum.max(max);
            }
        }

        dp[i] = maximum + 1;
        hashmap.insert(num, dp[i]);
        res = res.max(dp[i]);
    }

    res
}

pub fn main() {
    let s = "acfgbd".to_string();
    let k = 2;
    println!("{}", longest_ideal_string(s, k));
}
