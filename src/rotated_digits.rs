fn rotated_digits(n: i32) -> i32 {
    let mut count = 0;
    let invalid_set = ["3","4","7"];
    let set = ["2", "5", "6", "9"];
    for num in 0..=n {
        let num = num.to_string();
        if invalid_set.iter().any(|&v| num.contains(v)) {
            continue;
        }
        if set.iter().any(|v| num.contains(v)) {
            count += 1;
        }
    }

    count
}

pub fn main() {
    let n = 30;
    println!("{}", rotated_digits(n));
}
