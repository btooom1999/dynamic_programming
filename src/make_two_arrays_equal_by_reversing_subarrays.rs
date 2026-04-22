fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
    target.sort();
    arr.sort();

    target == arr
}

pub fn main() {
    let target = [1,2,3,4].to_vec();
    let arr = [2,4,1,3].to_vec();
    println!("{:?}", can_be_equal(target, arr));
}
