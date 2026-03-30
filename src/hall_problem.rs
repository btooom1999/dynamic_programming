fn find_nearest_valid_time(i: usize, k: &[Vec<i32>], p: &mut [i32]) {
    for j in 1..i {
        if k[i - 1][0] >= k[j - 1][1] {
            p[i] = j as i32;
        }
    }
}

pub fn main() {
    let n = 12;
    // let mut k = [
    //     vec![1, 3],
    //     vec![4, 6],
    //     vec![2, 5],
    //     vec![6, 7],
    //
    // ].to_vec();
    let mut k = [
        vec![1, 2],
        vec![3, 5],
        vec![0, 4],
        vec![6, 8],
        vec![7, 13],
        vec![4, 6],
        vec![9, 10],
        vec![9, 12],
        vec![11, 14],
        vec![15, 19],
        vec![14, 16],
        vec![18, 20],
    ]
    .to_vec();

    k.sort_by_key(|v| v[1]);

    let mut dp = vec![0; n + 1];

    let mut p = vec![0; n + 1];
    p[0] = 0;
    p[1] = 0;

    for i in 2..=n {
        find_nearest_valid_time(i, &k, &mut p);
    }

    for i in 1..=n {
        dp[i] = std::cmp::max(dp[i - 1], dp[p[i] as usize] + k[i - 1][1] - k[i - 1][0]);
    }

    println!("{:?}", dp[n]);
}
