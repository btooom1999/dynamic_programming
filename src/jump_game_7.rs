fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
    let s = s.as_bytes();
    let n = s.len();
    if s[n-1] == b'1' {
        return false;
    }

    let min_jump = min_jump as usize;
    let max_jump = max_jump as usize;
    let mut dp = vec![false; n];
    dp[n-1] = true;

    let mut step = 0;
    for i in (0..n-1).rev() {
        if i >= min_jump && dp[i-min_jump] {
            step += 1;
        }
        if i > max_jump && dp[i-max_jump-1] {
            step -= 1;
        }
        if s[i] == b'0' && step > 0 {
            dp[i] = true;
        }
    }

    dp[n-1]
}

pub fn main() {
    let s = "00".to_string();
    let max_jump = 1;
    let min_jump = 1;
    println!("{}", can_reach(s, min_jump, max_jump));
}
