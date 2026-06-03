use std::collections::HashMap;

fn dfs(
    expression: &str,
    memo: &mut HashMap<String, Vec<i32>>,
) -> Vec<i32> {
    if let Some(data) = memo.get(expression) {
        return data.clone();
    }

    let mut res = Vec::new();
    for (i, c) in expression.chars().enumerate() {
        if c == '*' || c == '+' || c == '-' {
            #[allow(clippy::char_indices_as_byte_indices)]
            let left = &expression[..i];
            let right = &expression[i+1..];

            let left_results = dfs(left, memo);
            let right_results = dfs(right, memo);

            for num1 in &left_results {
                for num2 in &right_results {
                    let val = match c {
                        '+' => num1 + num2,
                        '-' => num1 - num2,
                        '*' => num1 * num2,
                        _ => unreachable!(),
                    };
                    res.push(val);
                }
            }
        }
    }

    if res.is_empty() {
        res.push(expression.parse::<i32>().unwrap());
    }

    memo.insert(expression.to_string(), res.clone());
    res
}

fn diff_ways_to_compute(expression: String) -> Vec<i32> {
    dfs(&expression, &mut HashMap::new())
}

pub fn main() {
    // let expression = "2-1-1".to_string();
    // let expression = "2*3-4*5".to_string();
    let expression = "1-1-1-1-1".to_string();
    println!("{:?}", diff_ways_to_compute(expression));
}
