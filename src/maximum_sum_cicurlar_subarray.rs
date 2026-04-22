fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut min = 0;
    let mut total = 0;
    let mut global_max = i32::MIN;
    let mut global_min = i32::MAX;

    for num in nums {
        total += num;
        max = num.max(num+max);
        min = num.min(num+min);
        global_max = global_max.max(max);
        global_min = global_min.min(min);
    }

    if global_max < 0 {
        return global_max;
    }

    global_max.max(total-global_min)
}

pub fn main() {
    let nums = [-3,-2,-3].to_vec();
    println!("{}", max_subarray_sum_circular(nums));
}
