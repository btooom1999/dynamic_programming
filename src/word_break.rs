fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut dp = vec![false; s.len() + 1];
    dp[s.len()] = true;

    for i in (0..s.len()).rev() {
        for w in &word_dict {
            if i + w.len() <= s.len() && &s[i..i+w.len()] == w {
                dp[i] = dp[i + w.len()];
            }
            if dp[i] {
                break;
            }
        }
    }

    dp[0]
}

pub fn main() {
    let s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string();
    let word_dict = ["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"].into_iter().map(String::from).collect::<Vec<_>>();
    println!("{}", word_break(s, word_dict));
}
