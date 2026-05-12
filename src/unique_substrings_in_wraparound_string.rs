fn find_substring_in_wrapround_string(mut s: String) -> i32 {
    s.push('*');

    let s = s.as_bytes();
    let mut dp = vec![0; 26];

    let mut i = 0;
    for j in 0..s.len()-1 {
        let mut current = s[j] + 1;
        if current > b'z' { current = current % 123 + b'a'; }
        if current != s[j+1] {
            while i <= j {
                let k = (s[i]-b'a') as usize;
                dp[k] = dp[k].max((j-i+1) as i32);
                i += 1;
            }
        }
    }

    dp.into_iter().sum()
}

pub fn main() {
    let s = "zab".to_string();
    println!("{}", find_substring_in_wrapround_string(s));
}
