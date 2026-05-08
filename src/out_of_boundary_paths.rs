const DIRECTIONS: [(i32, i32,); 4] = [(0,1), (0,-1), (1,0), (-1,0)];
const MOD: i64 = 1_000_000_007;

fn dfs(
    x: i32,
    y: i32,
    m: i32,
    n: i32,
    step: i32,
    dp: &mut Vec<Vec<Vec<i64>>>,
) -> i64 {
    if x < 0 || x == m || y < 0 || y == n || step < 0 {
        return (step == 0) as i64;
    }

    let x = x as usize;
    let y = y as usize;
    let step = step as usize;

    if dp[x][y][step] == -1 {
        for direct in DIRECTIONS {
            let nx = direct.0 + x as i32;
            let ny = direct.1 + y as i32;
            dp[x][y][step] = (dp[x][y][step].max(0) + dfs(nx, ny, m, n, step as i32 - 1, dp)) % MOD;
        }
    }

    dp[x][y][step]
}

fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
    let mut dp = vec![vec![vec![-1; (max_move+1) as usize]; n as usize]; m as usize];
    dp[start_row as usize][start_column as usize][0] = 0;

    for step in 1..max_move+1 {
        dfs(start_row, start_column, m, n, step, &mut dp);
    }

    (dp[start_row as usize][start_column as usize].iter().sum::<i64>() % MOD) as i32
}

pub fn main() {
    let m = 50;
    let n = 50;
    let max_move = 50;
    let start_row = 25;
    let start_column = 25;
    println!("{}", find_paths(m, n, max_move, start_row, start_column));
}
