use std::collections::HashMap;

fn transform(num: i32, memo: &mut HashMap<i32, i32>) -> i32 {
    if num == 1 {
        return 0;
    }

    if let Some(&steps) = memo.get(&num) {
        return steps;
    }

    let mut value = 1;
    if num % 2 == 0 {
        value += transform(num/2, memo);
    } else {
        value += transform(3*num+1, memo);
    }

    memo.insert(num, value);
    value
}

fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
    let mut res = Vec::with_capacity((hi-lo+1) as usize);
    for num in lo..hi+1 {
        let steps = transform(num, &mut HashMap::new());
        res.push((num, steps));
    }

    res.sort_by_key(|v| v.1);
    res[k as usize-1].0
}

pub fn main() {
    let lo = 1;
    let hi = 1000;
    let k = 777;
    println!("{}", get_kth(lo, hi, k));
}
