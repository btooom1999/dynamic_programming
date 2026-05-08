fn min_flips_mono_incr(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut prefix_ones = vec![0; n+1];

    for i in 0..n {
        prefix_ones[i+1] = prefix_ones[i] + (s[i] == b'1') as i32;
    }

    let mut res = i32::MAX;
    for i in 0..n+1 {
        let zeroes = (n-i) as i32 - (prefix_ones[n]-prefix_ones[i]);
        res = res.min(prefix_ones[i]+zeroes);
    }

    res
}

pub fn main() {
    let s = "00110".to_string();
    println!("{}", min_flips_mono_incr(s));
}
