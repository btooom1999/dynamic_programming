const DIRECTIONS: [(isize, isize); 4] = [(1,0), (-1,0), (0,1), (0,-1)];

#[allow(clippy::too_many_arguments)]
fn dfs(
    target: i32,
    amount: i32,
    classroom: &Vec<Vec<u8>>,
    default_energy: i32,
    energy: i32,
    step: i32,
    visited: &mut Vec<Vec<bool>>,
    dp: &mut Vec<Vec<Vec<i32>>>,
    x: usize,
    y: usize,
    m: usize,
    n: usize,
) -> i32 {
    if energy == 0 || target == amount {
        return if target == amount { step } else { -1 };
    }

    if dp[x][y][energy as usize] != -2 {
        return dp[x][y][energy as usize];
    }

    let mut min = i32::MAX;
    for direct in DIRECTIONS {
        if let (Some(x), Some(y)) = (x.checked_add_signed(direct.0), y.checked_add_signed(direct.1))
            && x < m
            && y < n
            && !visited[x][y]
            && classroom[x][y] != b'X'
        {
            visited[x][y] = true;
            let energy = if classroom[x][y] == b'R' { default_energy } else { energy-1 };
            let cleaned = (classroom[x][y] == b'L') as i32;
            let val = dfs(target, amount+cleaned, classroom, default_energy, energy, step+1, visited, dp, x, y, m, n );
            if val != i32::MAX {
                min = min.min(val);
            }
            visited[x][y] = false;
        }
    }

    if min == i32::MAX { min = -1 };
    dp[x][y][energy as usize] = min;
    min
}

fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
    let classroom = classroom.into_iter().map(|s| s.into_bytes()).collect::<Vec<_>>();
    let (m, n) = (classroom.len(), classroom[0].len());

    let mut target = 0;
    let (mut x, mut y) = (0, 0);
    for i in 0..m {
        for j in 0..n {
            if classroom[i][j] == b'L' {
                target += 1;
            } else if classroom[i][j] == b'S' {
                x = i;
                y = j;
            }
        }
    }

    let mut visited = vec![vec![false; n]; m];
    visited[x][y] = true;
    dfs(target, 0, &classroom, energy, energy, 0, &mut visited, &mut vec![vec![vec![-2; (energy+1) as usize]; n]; m], x, y, m, n)
}

pub fn main() {
    let classroom = ["LS", "RL"].into_iter().map(String::from).collect();
    let energy = 4;
    println!("{}", min_moves(classroom, energy));
}
