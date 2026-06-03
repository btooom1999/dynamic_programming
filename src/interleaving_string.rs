fn dfs(
    s1: &[u8],
    s2: &[u8],
    s3: &[u8],
    x: usize,
    y: usize,
    z: usize,
    dp: &mut Vec<Vec<bool>>,
) -> bool {
    if z == s3.len() {
        return true;
    }

    if dp[x][y] {
        return false;
    }

    let mut flag = false;
    if x < s1.len() && s1[x] == s3[z] {
        flag = dfs(s1, s2, s3, x+1, y, z+1, dp);
    }

    if y < s2.len() && s2[y] == s3[z] {
        flag = flag || dfs(s1, s2, s3, x, y+1, z+1, dp);
    }


    dp[x][y] = true;
    flag
}

fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    if s1.len() + s2.len() != s3.len() {
        return false;
    }

    let mut dp = vec![vec![false; s2.len()+1]; s1.len()+1];
    dfs(s1.as_bytes(), s2.as_bytes(), s3.as_bytes(), 0, 0, 0, &mut dp)
}

pub fn main() {
    let s1 = "aabcc".to_string();
    let s2 = "dbbca".to_string();
    let s3 = "aadbbcbcac".to_string();
    println!("{}", is_interleave(s1, s2, s3));
}
