fn dfs(
    visited: &mut i32,
    n: usize,
    i: usize,
) -> i32 {
    if i > n {
        return 1;
    }

    let mut count = 0;
    for num in 1..=n {
        if 1 << num & *visited > 0 && (i % num == 0 || num % i == 0) {
            *visited ^= 1 << num;
            count += dfs(visited, n, i+1);
            *visited ^= 1 << num;
        }
    }

    count
}


fn count_arrangement(n: i32) -> i32 {
    dfs(&mut ((1 << (n+1))-1), n as usize, 1)
}

pub fn main() {
    let n = 15;
    println!("{}", count_arrangement(n));
}
