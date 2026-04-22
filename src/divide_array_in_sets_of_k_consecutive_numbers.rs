use std::collections::{HashMap, VecDeque};

fn is_possible_divide(mut hand: Vec<i32>, group_size: i32) -> bool {
    if group_size == 1 {
        return true;
    }

    hand.sort();
    let mut hashmap = HashMap::<i32, VecDeque<i32>>::new();
    for num in hand {
        if hashmap.contains_key(&(num-1)) {
            let data = hashmap.get_mut(&(num-1)).unwrap();
            let front = data.pop_front().unwrap();

            if data.is_empty() {
                hashmap.remove(&(num-1));
            }

            if front < group_size-1 {
                hashmap.entry(num).or_default().push_back(front+1);
            }

        } else {
            hashmap.entry(num).or_default().push_back(1);
        }
    }

    hashmap.is_empty()
}

pub fn main() {
    let hand = [1,2,3,3,4,4,5,6].to_vec();
    let group_size = 4;
    println!("{}", is_possible_divide(hand, group_size));
}
