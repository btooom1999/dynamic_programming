fn dfs(
    index: usize,
    triangle: &Vec<Vec<i32>>,
    dp: Vec<i32>,
) -> Vec<i32> {
    let mut res = Vec::new();
    for i in 0..triangle[index].len() {
        let num = triangle[index][i];
        res.push((num+dp[i]).min(num+dp[i+1]));
    }

    if index == 0 {
        res
    } else {
        dfs(index-1, triangle, res)
    }
}

fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    dfs(triangle.len()-1, &triangle, vec![0; triangle[triangle.len()-1].len()+1])[0]
}

pub fn main() {
    let triangle = [vec![2], vec![3,4], vec![6,5,7], vec![4,1,8,3]];
    println!("{}", minimum_total(triangle.into()));
}
