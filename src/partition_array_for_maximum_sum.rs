use std::collections::HashMap;

fn dfs(
    idx: usize,
    k: usize,
    arr: &Vec<i32>,
    hashmap: &mut HashMap<usize, i32>,
) -> i32 {
    if idx >= arr.len() {
        return 0;
    }

    if let Some(&max) = hashmap.get(&idx) {
        return max;
    }

    let mut res = arr[idx] + dfs(idx+1, k, arr, hashmap);
    let mut max = 0;
    for (n, &num) in arr.iter().skip(idx).take(k).enumerate() {
        max = std::cmp::max(max, num);
        res = res.max(max * (n+1) as i32 + dfs(idx+n+1, k, arr, hashmap));
    }

    hashmap.insert(idx, res);
    res
}

fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
    dfs(0, k as usize, &arr, &mut HashMap::new())
}

pub fn main() {
    let arr = [1,4,1,5,7,3,6,1,9,9,3];
    let k = 4;
    println!("{}", max_sum_after_partitioning(arr.into(), k));
}
