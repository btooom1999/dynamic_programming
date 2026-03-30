use std::collections::HashMap;

fn fibonacci(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    if n <= 0 {
        return 0;
    }
    if memo.contains_key(&n) {
        return *memo.get(&n).unwrap();
    }

    let number = fibonacci(n - 1, memo) + fibonacci(n - 2, memo);
    memo.insert(n, number);

    number
}

pub fn main() {
    let n = 6;

    let mut memo = HashMap::from([(0, 0), (1, 1)]);

    fibonacci(n, &mut memo);

    for (key, val) in memo.iter() {
        println!("Key {} with value {} ", key, val);
    }
}
