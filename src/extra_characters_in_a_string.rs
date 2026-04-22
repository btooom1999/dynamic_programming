use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
}

impl TrieNode {
    fn new() -> Self {
        const NONE: Option<Box<TrieNode>> = None;
        Self { children: [NONE; 26] }
    }
}

fn dfs(
    idx: usize,
    hashset: &HashSet<String>,
    trie: &mut Box<TrieNode>,
    s: &[u8],
    dp: &mut Vec<i32>,
) -> i32 {
    if dp[idx] > -1 {
        return dp[idx];
    }

    let mut res = i32::MAX;
    for i in idx..s.len() {
        let mut cur_trie = trie.as_mut();
        let mut str = String::new();
        let mut j = i;

        let mut valid_strings = Vec::new();
        while j < s.len() && cur_trie.children[(s[j]-b'a') as usize].is_some() {
            str.push(s[j] as char);
            cur_trie = cur_trie.children[(s[j]-b'a') as usize].as_mut().unwrap();
            j += 1;
            if hashset.contains(&str) {
                valid_strings.push(j);
            }
        }

        let count = (i - idx) as i32;
        for j in valid_strings {
            res = res.min(count + dfs(j, hashset, trie, s, dp));
        }

        res = res.min(count + 1 + dfs(i+1, hashset, trie, s, dp));
    }

    dp[idx] = res;
    res
}

fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
    let mut hashset = HashSet::new();
    let mut trie = Box::new(TrieNode::new());

    for word in dictionary {
        hashset.insert(word.clone());
        let mut trie = &mut trie;
        for b in word.as_bytes() {
            let k = (b - b'a') as usize;
            trie = trie.children[k].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
    }

    let mut dp = vec![-1; s.len()+1];
    dp[s.len()] = 0;
    dfs(0, &hashset, &mut trie, s.as_bytes(), &mut dp)
}

pub fn main() {
    // let s = "sayhelloworld".to_string();
    // let dictionary = ["hello","world"].into_iter().map(String::from).collect();
    let s = "leetscode".to_string();
    let dictionary = ["leet","code","leetcode"].into_iter().map(String::from).collect();
    println!("{}", min_extra_char(s, dictionary));
}
