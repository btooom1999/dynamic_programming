use std::collections::HashMap;

fn dfs(
    i: usize,
    m: i32,
    n: i32,
    data: &[[i32; 2]],
    memo: &mut HashMap<(usize, i32, i32), i32>,
) -> i32 {
    if i == data.len() || (m == 0 && n == 0) {
        return 0;
    }

    if let Some(&max) = memo.get(&(i, m, n)) {
        return max;
    }

    let mut res = dfs(i+1, m, n, data, memo);
    let (zeroes, ones) = (data[i][0], data[i][1]);
    if m >= zeroes && n >= ones {
        res = res.max(1+dfs(i+1, m-zeroes, n-ones, data, memo));
    }

    memo.insert((i, m, n), res);
    res
}

fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    let mut data: Vec<[i32; 2]> = Vec::with_capacity(2);

    for str in strs {
        let mut ones = 0;
        let mut zeroes = 0;
        for c in str.chars() {
            if c == '1' {
                ones += 1;
            } else {
                zeroes += 1;
            }
        }

        data.push([zeroes, ones]);
    }

    let mut memo = HashMap::new();
    dfs(0, m, n, &data, &mut memo)
}

pub fn main() {
    // let strs = ["10","0001","111001","1","0"].into_iter().map(String::from).collect();
    let strs = ["101000000","1100001010","11101000","011010110","0010001","0011","0111101111"].into_iter().map(String::from).collect();
    let m = 10;
    let n = 11;
    println!("{}", find_max_form(strs, m, n));
}
