use std::collections::HashMap;

fn dfs(
    hashmap: &mut HashMap<(i32, i32), i32>,
    data: &Vec<(i32, i32)>,
    i: i32,
    j: i32,
) -> i32 {
    if i == data.len() as i32 {
        return 0;
    }

    if let Some(&max) = hashmap.get(&(i, j)) {
        return max;
    }

    let prev = if j < 0 { (0, 0) } else { data[j as usize] };
    let cur = data[i as usize];

    let mut res = 0;
    if !(cur.0 > prev.0 && cur.1 < prev.1) {
        res = dfs(hashmap, data, i+1, i) + cur.0;
    }

    res = res.max(dfs(hashmap, data, i+1, j));
    hashmap.insert((i, j), res);

    res
}

fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
    let mut data = scores.into_iter().zip(ages).collect::<Vec<_>>();
    data.sort();

    dfs(&mut HashMap::new(), &data, 0, -1)
}

pub fn main() {
    let scores = [596,277,897,622,500,299,34,536,797,32,264,948,645,537,83,589,770];
    let ages = [18 ,52 ,60 ,79 ,72 ,28 ,81,33 ,96 ,15,18 ,5  ,17 ,96 ,57 ,72,72];
    println!("{}", best_team_score(scores.into(), ages.into()));
}
