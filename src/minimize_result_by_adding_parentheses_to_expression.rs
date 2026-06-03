fn minimize_result(expression: String) -> String {
    let mut splited_expression = expression.split('+');
    let left = splited_expression.next().unwrap().as_bytes();
    let right = splited_expression.next().unwrap().as_bytes();
    let mut res = (i32::MAX, format!("({})", expression.clone()));
    for i in 0..left.len() {
        for j in 0..right.len() {
            let num1 = &left[0..i];
            let num3 = &right[0..j];

            let val1 = String::from_utf8(left[i..left.len()].to_vec()).unwrap().parse::<i32>().unwrap();
            let val2 = String::from_utf8(right[j..right.len()].to_vec()).unwrap().parse::<i32>().unwrap();

            match (num1.is_empty(), num3.is_empty()) {
                (true, true) => {
                    res.0 = val1 + val2;
                }
                (true, false) => {
                    let product = val2;
                    let val2 = String::from_utf8(num3.to_vec()).unwrap().parse::<i32>().unwrap();
                    let result = (val1 + val2) * product;
                    if res.0 > result {
                        res.0 = result;
                        res.1 = format!("({}+{}){}", val1, val2, product);
                    }
                }
                (false, true) => {
                    let product = String::from_utf8(num1.to_vec()).unwrap().parse::<i32>().unwrap();
                    let result = product * (val1 + val2);
                    if res.0 > result {
                        res.0 = result;
                        res.1 = format!("{}({}+{})", product, val1, val2);
                    }
                }
                _ => {
                    let product1 = String::from_utf8(num1.to_vec()).unwrap().parse::<i32>().unwrap();
                    let product2 = val2;
                    let val2 = String::from_utf8(num3.to_vec()).unwrap().parse::<i32>().unwrap();
                    let result = product1 * (val1 + val2) * product2;
                    if res.0 > result {
                        res.0 = result;
                        res.1 = format!("{}({}+{}){}", product1, val1, val2, product2);
                    }
                }
            }
        }
    }

    res.1
}

pub fn main() {
    // let expression = "12+34".to_string();
    let expression = "247+38".to_string();
    println!("{}", minimize_result(expression));
}
