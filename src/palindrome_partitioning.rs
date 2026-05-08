fn is_palindrome(str: &[u8]) -> bool {
    let mut i = 0;
    let mut j = str.len()-1;

    while i < j {
        if str[i] != str[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }

    true
}

fn dfs(
    i: usize,
    s: &[u8],
    memo: &mut Vec<Vec<Vec<String>>>,
) -> Vec<Vec<String>> {
    if i == s.len() {
        return vec![vec![]];
    }

    if memo[i].is_empty() {
        dfs(i+1, s, memo);
        let mut str = String::new();
        for idx in i..s.len() {
            str.push(s[idx] as char);
            if is_palindrome(str.as_bytes()) {
                for suffix in dfs(idx+1, s, memo) {
                    let mut substring = Vec::from([str.clone()]);
                    substring.extend(suffix);
                    memo[i].push(substring);
                }
            }
        }
    }

    memo[i].clone()
}

fn partition(s: String) -> Vec<Vec<String>> {
    dfs(0, s.as_bytes(), &mut vec![vec![]; s.len()])
}

pub fn main() {
    let s = "aabbax".to_string();
    println!("{:?}", partition(s));
}
