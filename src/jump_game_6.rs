use std::collections::BinaryHeap;

fn max_result(nums: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let n = nums.len();
    let mut max_heap = BinaryHeap::new();
    let mut res = nums[n-1];
    max_heap.push((res, n-1));

    for i in (0..n-1).rev() {
        while let Some(peek) = max_heap.peek() && peek.1 > i+k {
            max_heap.pop();
        }

        res = nums[i]+max_heap.peek().unwrap().0;
        max_heap.push((res, i));
    }

    res
}

pub fn main() {
    let nums = [1,-1,-2,4,-7,3].to_vec();
    let k = 2;
    println!("{}", max_result(nums, k));
}
