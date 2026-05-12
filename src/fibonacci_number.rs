fn fib(n: i32) -> i32 {
    if n < 2 {
        return (n == 1) as i32;
    }

    let mut f1 = 0;
    let mut f2 = 1;
    let mut k = 2;
    while k <= n {
        let f3 = f1 + f2;
        f1 = f2;
        f2 = f3;
        k += 1;
    }

    f2
}

pub fn main() {
    let n = 2;
    println!("{}", fib(n));
}
