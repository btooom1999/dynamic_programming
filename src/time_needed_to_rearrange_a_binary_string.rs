fn seconds_to_remove_occurrences(s: String) -> i32 {
    let mut zeroes = 0;
    let mut swap = 0;

    for c in s.chars() {
        zeroes += (c == '0') as i32;

        if c == '1' && zeroes > 0 {
            swap = std::cmp::max(swap+1, zeroes);
        }
    }

    swap
}

pub fn main() {
    let s = "0110101".to_string();
    println!("{}", seconds_to_remove_occurrences(s));
}
