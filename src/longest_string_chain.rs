use std::collections::HashMap;

fn dfs(
    hashmap: &HashMap <String, usize>,
    dp: &mut HashMap<String, i32>,
    words: &Vec<String>,
    idx: usize,
    max: &mut i32,
) -> i32 {
    if let Some(&max) = dp.get(&words[idx]) {
        return max;
    }

    let mut res = 1;
    for i in 0..words[idx].len() {
        let mut new_str = words[idx].clone();
        new_str.remove(i);

        if let Some(&idx) = hashmap.get(&new_str) {
            res = res.max(1 + dfs(hashmap, dp, words, idx, max));
        }
    }

    if idx > 0 {
        dfs(hashmap, dp, words, idx-1, max);
    }

    *max = std::cmp::max(*max, res);
    dp.insert(words[idx].clone(), res);
    res
}

fn longest_str_chain(mut words: Vec<String>) -> i32 {
    words.sort_by_key(|a| a.len());

    let mut hashmap = HashMap::new();
    for (i, word) in words.iter().enumerate() {
        hashmap.insert(word.to_owned(), i);
    }

    let mut max = 0;
    dfs(&hashmap, &mut HashMap::new(), &words, words.len()-1, &mut max);
    max
}

pub fn main() {
    let words = ["a","b","ba","abc","abd","bdca"].into_iter().map(String::from).collect();
    println!("{}", longest_str_chain(words));
}
