fn can_jump(nums: Vec<i32>) -> bool {
    let mut max = 0;
    for i in 0..nums.len() {
        max = max.max(i + nums[i] as usize);

        if max >= nums.len()-1 {
            return true;
        }

        if i == max {
            return false;
        }
    }

    unreachable!()
}

pub fn main() {
    // let nums = [3,2,1,0,4].to_vec();
    let nums = [2,3,1,1,4].to_vec();
    println!("{}", can_jump(nums));
}
