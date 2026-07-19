fn can_partition(nums: Vec<i32>) -> bool {
    let sum = nums.iter().sum::<i32>();
    if sum % 2 != 0 { return false; }
    let target = (sum / 2) as usize;
    let n = (target / 64) + 1;
    let mut dp = vec![0u64; n];
    dp[0] = 1;

    for num in nums {
        if num > target as i32 { continue; }

        let idx = (num / 64) as usize;
        let offset = (num % 64) as usize;
        for dist_idx in (idx..n).rev() {
            let src_idx = dist_idx - idx;
            let mut shifted = dp[src_idx] << offset;
            if src_idx > 0 && offset > 0 {
                shifted |= dp[src_idx-1] >> (64 - offset);
            }
            dp[dist_idx] |= shifted;
        }

        if dp[target / 64] >> (target % 64) & 1 == 1 {
            return true;
        }
    }

    dp[target / 64] >> (target % 64) & 1 == 1
}

pub fn main() {
    let nums = [64,64].to_vec();
    println!("{}", can_partition(nums));
}
