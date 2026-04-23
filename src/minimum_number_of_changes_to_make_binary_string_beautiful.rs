fn min_changes(s: String) -> i32 {
    let mut res = 0;
    for b in s.as_bytes().chunks(2) {
        if b[0] != b[1] {
            res += 1;
        }
    }

    res
}

pub fn main() {
    let s = "1001".to_string();
    println!("{}", min_changes(s));
}
