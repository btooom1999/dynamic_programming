use std::collections::VecDeque;

fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    let n = n as usize;

    let src = src as usize;
    let dst = dst as usize;

    let mut map = vec![vec![]; n];
    for flight in &flights {
        map[flight[1] as usize].push((flight[0] as usize, flight[2]));
    }

    let mut res = i32::MAX;
    let mut queue = map[dst].clone().into_iter().map(|v| (v.0, v.1, 0)).collect::<VecDeque<_>>();

    while let Some((from, price, count)) = queue.pop_front() {
        if from == src {
            res = res.min(price);
        }

        if count == k {
            continue;
        }

        for data in &map[from] {
            queue.push_front((data.0, data.1+price, count+1));
        }
    }

    if res == i32::MAX { -1 } else { res }
}

pub fn main() {
    let n = 5;
    let flights = [[1,0,5],[2,1,5],[3,0,2],[1,3,2],[4,1,1],[2,4,1]].into_iter().map(|v| v.to_vec()).collect::<Vec<_>>();
    let src = 2;
    let dst = 0;
    let k = 2;
    println!("{}", find_cheapest_price(n, flights, src, dst, k));
}
