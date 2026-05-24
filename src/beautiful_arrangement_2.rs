fn construct_array(n: i32, mut k: i32) -> Vec<i32> {
    let mut res = (1..=n).collect::<Vec<_>>();
    for i in 1..n as usize {
        if k == 0 {
            break;
        }
        if i % 2 > 0 {
            res[i] = res[i-1] + k;
        } else {
            res[i] = res[i-1] - k;
        }
        k -= 1;
    }

    res
}

pub fn main() {
    let n = 15;
    let k = 5;
    println!("{:?}", construct_array(n, k));
}
