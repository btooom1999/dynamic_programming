fn maximum_cost_substring(s: String, chars: String, vals: Vec<i32>) -> i32 {
    let mut values = (1..=26).collect::<Vec<_>>();
    for (i, c) in chars.chars().enumerate() {
        let k = (c as u8 - b'a') as usize;
        values[k] = vals[i];
    }

    let s = s.as_bytes();
    let mut max = values[(s[0]-b'a') as usize];
    let mut res = max;

    for i in 1..s.len() {
        let value = values[(s[i]-b'a') as usize];
        max = value.max(max + value);
        res = res.max(max);
    }

    res.max(0)
}

pub fn main() {
    let s = "adaa".to_string();
    let chars = "d".to_string();
    let vals = [-1000].to_vec();
    println!("{}", maximum_cost_substring(s, chars, vals));
}
