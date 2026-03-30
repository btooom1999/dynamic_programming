fn longest_palindrome(s: String) -> String {
    let mut dp = vec![vec![false; s.len()]; s.len()];

    let mut from = 0;
    let mut to = 0;
    let s = s.as_bytes();
    let n = s.len();
    for i in (0..n).rev() {
        for j in i..n {
            if s[i] == s[j] && (j - i <= 2 || dp[i+1][j-1]) {
                dp[i][j] = true;
                if j-i > to-from {
                    from = i;
                    to = j;
                }
            }
        }
    }

    String::from_utf8(s[from..=to].to_vec()).unwrap()
}

pub fn main() {
    let s = "aaaa".to_string();
    println!("{}", longest_palindrome(s));
}
