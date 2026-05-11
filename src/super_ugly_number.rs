fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
    let n = n as usize;
    let k = primes.len();
    let mut next = primes.clone();
    let mut indices = vec![0; k];
    let mut res = Vec::with_capacity(n);
    res.push(1);

    for _ in 0..n {
        let min = *next.iter().min().unwrap();
        res.push(min);

        for i in 0..k {
            if next[i] == min {
                indices[i] += 1;
                next[i] = primes[i].saturating_mul(res[indices[i]]);
            }
        }
    }

    res[n-1]
}

pub fn main() {
    let n = 12;
    let primes = [2,7,13,19].to_vec();
    println!("{}", nth_super_ugly_number(n, primes));
}
