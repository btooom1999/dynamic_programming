fn count_numbers_with_unique_digits(n: i32) -> i32 {
    let mut f = 1;
    let mut sum = 1;
    for i in 1..=n {
        sum *= (11 - i).min(9);
        f += sum;
    }

    f
}

pub fn main() {
    let n = 8;
    println!("{}", count_numbers_with_unique_digits(n));
}
