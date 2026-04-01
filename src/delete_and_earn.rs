fn delete_and_earn(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    let n = nums.len();

    let mut max = 0;
    let mut dp = vec![0;nums.len()];
    for i in (0..n).rev() {
        let num = nums[i];
        dp[i] += num;

        let mut x = i+1;
        while x<n && nums[x]-1 == num {
            x += 1;
        }

        let mut y = x;
        while x<n && y+1<n && nums[y] == nums[x] {
            y += 1;
        }

        if x < n && y < n && nums[x]-1 != num && nums[y]-1 != num && nums[x] != nums[y] {
            dp[i] += dp[x].max(dp[y]);
        } else if x < n && nums[x]-1 != num {
            dp[i] += dp[x];
        }

        max = std::cmp::max(dp[i], max);
    }

    max
}

pub fn main() {
    let nums = [1,1,1,2,4,5,5,5,6];
    println!("{}", delete_and_earn(nums.into()));
}
