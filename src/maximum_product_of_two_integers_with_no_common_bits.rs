fn max_product(nums: Vec<i32>) -> i64 {
    let max = *nums.iter().max().unwrap();
    let mut dp = (0..max+1).collect::<Vec<_>>();
    let mut hashset = vec![false; (max+1) as usize];
    hashset[0] = true;

    for &num in &nums {
        hashset[num as usize] = true;
    }

    for num in 1..max {
        if !hashset[num as usize] {
            dp[num as usize] = 0;

            for i in 0..32 {
                if num >> i & 1 == 1 {
                    dp[num as usize] = dp[num as usize].max(dp[(num ^ (1 << i)) as usize]);
                }
            }
        }
    }

    let mut res = 0;
    for num in nums {
        let n = 32 - num.leading_zeros();
        let mask = (1 << n) - 1;
        res = res.max(num as i64 * dp[(num ^ mask) as usize] as i64);
    }

    res
}

fn generate_nums() -> Vec<i32> {
    (0..100_000).map(|_| rand::random_range(1..=1_000_000)).collect()
}

pub fn main() {
    let nums = [5,6,4].to_vec();
    // let nums = (0..100_000).map(|v| 1_000_000 - v).collect();
    println!("{}", max_product(nums));
}
