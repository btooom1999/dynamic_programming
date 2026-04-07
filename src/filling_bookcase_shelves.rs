fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
    let n = books.len();
    let mut dp = vec![0; n+1];

    for i in (0..n).rev() {
        let mut height = 0;
        let mut width = 0;
        let mut res = i32::MAX;
        for j in i..n {
            if width + books[j][0] > shelf_width {
                break;
            }

            width += books[j][0];
            height = height.max(books[j][1]);
            res = res.min(height + dp[j+1]);
        }

        dp[i] = res;
    }

    dp[0]
}

pub fn main() {
    let books = [[1,1],[2,3],[2,3],[1,1],[1,1],[1,1],[1,2]].into_iter().map(Vec::from).collect();
    let shelf_width = 4;
    // let books = [[1,3],[2,4],[3,2]].into_iter().map(Vec::from).collect();
    // let shelf_width = 6;
    println!("{}", min_height_shelves(books, shelf_width));
}
