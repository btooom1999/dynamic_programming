fn longest_ideal_string(s: String, k: i32) -> i32 {
    let n = s.len();
    let s = s.as_bytes();
    let mut hashmap = [0;26];
    hashmap[(s[n-1]-b'a') as usize] = 1;

    let mut res = 1;
    for i in (0..n-1).rev() {
        let num = (s[i]-b'a') as i32;
        let mut maximum = 0;
        for i in (num-k).max(0)..(num+k+1).min(26) {
            maximum = maximum.max(hashmap[i as usize]);
        }

        hashmap[num as usize] = maximum+1;
        res = res.max(maximum+1);
    }

    res
}

pub fn main() {
    let s = "acfgbd".to_string();
    let k = 2;
    println!("{}", longest_ideal_string(s, k));
}
