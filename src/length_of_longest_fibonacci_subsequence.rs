use std::collections::HashMap;

fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut dp = vec![vec![0; n]; n];
    let mut res = 0;

    let mut hashmap = HashMap::new();
    for i in 0..n {
        hashmap.insert(arr[i], i);
    }

    for i in (0..n-2).rev() {
        for j in i+1..n-1 {
            if let Some(&x) = hashmap.get(&(arr[i]+arr[j])) {
                dp[i][j] = 3.max(1+dp[j][x]);
                res = res.max(dp[i][j]);
            }
        }
    }

    res
}

pub fn main() {
    let arr = [1,2,3,4,5,6,7,8].to_vec();
    println!("{:?}", len_longest_fib_subseq(arr));
}
