use std::collections::HashMap;

fn dfs(
    data: &mut Vec<i32>,
    needs: &Vec<i32>,
    price: &Vec<i32>,
    hashmap: &HashMap<Vec<i32>, i32>,
    sum: i32,
    memo: &mut HashMap<Vec<i32>, i32>,
) -> i32 {
    let n = data.len();
    let mut min = if let Some(&offer) = hashmap.get(data) {
        let needs = (0..n).map(|i| needs[i] - data[i]).collect();

        offer + dfs(&mut vec![0; n], &needs, price, hashmap, 0, memo)
    } else {
        i32::MAX
    };

    if data == needs {
        return sum.min(min);
    }

    let mut k = data.clone();
    k.extend(needs.clone());

    if let Some(&min) = memo.get(&k) {
        return min;
    }

    for i in 0..n {
        if data[i] < needs[i] {
            data[i] += 1;
            min = min.min(dfs(data, needs, price, hashmap, sum + price[i], memo));
            data[i] -= 1;
        }
    }

    memo.insert(k, min);

    min
}

fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
    let n = price.len();
    let mut hashmap = HashMap::new();
    for offer in special {
        let first = offer[..n].to_vec();
        if let Some(price) = hashmap.get_mut(&first) {
            if *price > offer[n] {
                *price = offer[n];
            }
        } else {
            hashmap.insert(first, offer[n]);
        }
    }

    dfs(&mut vec![0; n], &needs, &price, &hashmap, 0, &mut HashMap::new())
}

fn generate_testcase() -> (Vec<i32>, Vec<Vec<i32>>, Vec<i32>) {
    let (range1, range2) = ((0..6), (0..11));
    let price = range1.clone().map(|_| rand::random_range(range2.clone())).collect();
    let needs = range1.clone().map(|_| rand::random_range(range2.clone())).collect();
    let special = (0..100).map(|_| {
        let mut res = range1.clone().map(|_| rand::random_range(range2.clone())).collect::<Vec<_>>();
        res.push(rand::random_range(0..51));
        res
    }).collect();

    (price, special, needs)
}

pub fn main() {
    let price = [1,1,1].to_vec();
    let special = [[1,1,0,0],[2,2,1,9]].map(|v| v.to_vec()).to_vec();
    let needs = [1,1,0].to_vec();
    // let (price, special, needs) = generate_testcase();
    // println!("{:?}", price);
    // println!("{:?}", special);
    // println!("{:?}", needs);
    println!("{}", shopping_offers(price, special, needs));
}
