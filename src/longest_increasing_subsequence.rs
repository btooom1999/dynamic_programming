pub fn main() {
    let arr = [1, 2, 5, 4, 6, 2];
    let mut f: Vec<i32> = Vec::new();

    let mut longest = 0;

    for i in 0..arr.len() {
        f.push(1);

        for j in 0..i {
            if arr[i] > arr[j] {
                f[i] = f[j] + 1;
            }
        }

        longest = longest.max(f[i]);
    }

    println!("{}", longest);
}
