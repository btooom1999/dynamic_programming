fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let mut dp = vec![vec![]; triangle.len()+1];
    dp[triangle.len()] = vec![0; triangle.len()+1];

    for i in (0..triangle.len()).rev() {
        for (j, num) in triangle[i].iter().enumerate() {
            let res = num + dp[i+1][j].min(dp[i+1][j+1]);
            dp[i].push(res);
        }
    }

    dp[0][0]
}

pub fn main() {
    let triangle = [vec![2], vec![3,4], vec![6,5,7], vec![4,1,8,3]];
    println!("{}", minimum_total(triangle.into()));
}
