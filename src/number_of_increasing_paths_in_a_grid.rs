const DIRECTIONS: [(i32, i32); 4] = [(1,0), (-1,0), (0,1), (0,-1)];
const MOD: i32 = 1_000_000_007;

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
            count = (count + dfs(matrix, ni, nj, m, n, memo)) % MOD;
        }
    }

    memo[i][j] = count;
    count
}

fn count_paths(matrix: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut memo = vec![vec![0; n]; m];
    let mut res = 0;
    for i in 0..m {
        for j in 0..n {
            res = (res + dfs(&matrix, i, j, m, n, &mut memo)) % MOD;
        }
    }

    res
}

pub fn main() {
    let matrix = [[1,1],[3,4]].into_iter().map(Vec::from).collect();
    println!("{}", count_paths(matrix));
}

