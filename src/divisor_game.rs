fn divisor_game(n: i32) -> bool {
    n % 2 == 0
}

pub fn main() {
    let n = 1000;
    println!("{}", divisor_game(n));
}
