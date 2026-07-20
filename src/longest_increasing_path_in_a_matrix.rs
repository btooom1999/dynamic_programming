const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

fn dfs(
    matrix: &Vec<Vec<i32>>,
    i: usize,
    j: usize,
    m: usize,
    n: usize,
    memo: &mut Vec<Vec<i32>>,
) -> i32 {
    if memo[i][j] > 0 {
        return memo[i][j];
    }

    let mut count = 1;
    for direct in DIRECTIONS {
        let ni = if direct.0 >= 0 { i + direct.0 as usize } else { i.wrapping_sub(1) };
        let nj = if direct.1 >= 0 { j + direct.1 as usize } else { j.wrapping_sub(1) };
        if ni < m && nj < n && matrix[i][j] < matrix[ni][nj] {
            count = count.max(1 + dfs(matrix, ni, nj, m, n, memo));
        }
    }

    memo[i][j] = count;
    count
}

fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut memo = vec![vec![0; n]; m];
    let mut res = 0;
    for i in 0..m {
        for j in 0..n {
            res = res.max(dfs(&matrix, i, j, m, n, &mut memo));
        }
    }

    res
}

pub fn main() {
    let matrix = [[9,9,4],[6,6,8],[2,1,1]].into_iter().map(Vec::from).collect();
    println!("{}", longest_increasing_path(matrix));
}
