fn count_substrings(s: String) -> i32 {
    let s = s.as_bytes();
    let mut dp = vec![vec![false; s.len()]; s.len()];
    let mut count = 0;
    let n = s.len();
    for i in (0..n).rev() {
        for j in i..n {
            if s[i] == s[j] && (j-i<=2 || dp[i+1][j-1]) {
                dp[i][j] = true;
                count += 1;
            }
        }
    }

    count
}

pub fn main() {
    let s = "aaa".to_string();
    println!("{}", count_substrings(s));
}
