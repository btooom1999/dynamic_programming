fn dfs(
    expression: &[u8],
) -> (i32, i32) {
    if expression == [b'-', b'x'] {
        return (-1, 0);
    }

    if expression == [b'x'] {
        return (1, 0);
    }

    let n = expression.len();
    if expression[n-1] == b'x' && let Ok(num) = String::from_utf8(expression[..n-1].to_vec()) && let Ok(num) = num.parse::<i32>() {
        (num, 0)
    } else if expression[n-1] != b'x' && let Ok(num) = String::from_utf8(expression[..n].to_vec()) && let Ok(num) = num.parse::<i32>() {
        (0, num)
    } else {
        let mut res = (0, 0);
        for i in (expression[0] == b'-') as usize..expression.len() {
            if expression[i] == b'+' || expression[i] == b'-' {
                let data1 = dfs(&expression[0..i]);
                let data2 = dfs(&expression[i + (expression[i] == b'+') as usize..]);
                res.0 += data1.0 + data2.0;
                res.1 += data1.1 + data2.1;
                break;
            }
        }

        res
    }
}

fn solve_equation(equation: String) -> String {
    let mut splitted = equation.split('=');
    let left = splitted.next().unwrap().as_bytes();
    let right = splitted.next().unwrap().as_bytes();
    let a = dfs(left);
    let b = dfs(right);

    if a.0-b.0 == 0 {
        if b.1-a.1 == 0 {
            return "Infinite solutions".to_string();
        } else {
            return "No solution".to_string();
        }
    }

    format!("x={}", (b.1-a.1)/(a.0-b.0))
}

pub fn main() {
    let equation = "3x=15".to_string();
    println!("{}", solve_equation(equation));
}
