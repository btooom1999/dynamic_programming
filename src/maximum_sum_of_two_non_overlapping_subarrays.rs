fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
    let (first_len, second_len) = (first_len as usize, second_len as usize);
    let mut sum = 0;
    let mut i = 0;
    let n = nums.len();
    let mut res = 0;
    for j in 0..n {
        sum += nums[j];
        if j >= first_len {
            sum -= nums[i];
            i += 1;
        }
        if j >= first_len-1 {
            if j+1-first_len > second_len {
                let mut left = 0;
                let mut x = 0;
                for y in 0..j+1-first_len {
                    left += nums[y];
                    if y >= second_len {
                        left -= nums[x];
                        x += 1;
                    }
                    res = res.max(sum + left);
                }
            }
            if n-j > second_len {
                let mut right = 0;
                let mut x = 1;
                for y in 1..n-j {
                    right += nums[j+y];
                    if y > second_len {
                        right -= nums[j+x];
                        x += 1;
                    }
                    res = res.max(sum + right);
                }
            }
        }
    }

    res
}

fn generate_random_nums() -> Vec<i32> {
    (0..1000).map(|_| rand::random_range(0..1001)).collect()
}

pub fn main() {
    // let nums = generate_random_nums();
    // let first_len = 234;
    // let second_len = 500;
    let nums = [1,0,3].to_vec();
    let first_len = 1;
    let second_len = 2;
    println!("{}", max_sum_two_no_overlap(nums, first_len, second_len))
}
