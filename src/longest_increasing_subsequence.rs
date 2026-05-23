fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut res = Vec::new();
    for num in nums {
        let last = res.last();
        if last.is_none_or(|&v| v < num) {
            res.push(num);
        } else {
            let i = res.partition_point(|&v| v < num);
            res[i] = num;
        }
    }

    res.len() as i32
}

pub fn main() {
    let nums = vec![1,3,6,7,9,4,10,5,6];
    // let nums = vec![10,9,2,5,3,7,101,18];
    println!("{}", length_of_lis(nums));
}
