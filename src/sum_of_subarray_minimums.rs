use std::collections::HashMap;

const MOD: i64 = 1_000_000_007;

fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut hashmap = HashMap::<(usize, usize), i32>::new();
    let mut sum = 0;
    for i in 0..n {
        sum = ((sum as i64 + arr[i] as i64) % MOD) as i32;
        hashmap.insert((i, i), arr[i]);
    }

    for i in 1..n {
        for j in 0..n - i {
            if let (Some(&val1), Some(&val2)) = (hashmap.get(&(j, j+i-1)), hashmap.get(&(j+1, j+i))) {
                let val = val1.min(val2);
                hashmap.insert((j, j+i), val);
                hashmap.remove(&(j, j+i-1));

                if j == n - i - 1 {
                    hashmap.remove(&(j+1, j+i));
                }
                sum = ((sum as i64 + val as i64) % MOD) as i32;
            }
        }
    }

    sum
}

pub fn main() {
    let arr = vec![11, 81, 94, 43, 3];
    println!("{}", sum_subarray_mins(arr));
}
