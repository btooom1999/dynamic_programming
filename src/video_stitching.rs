use std::collections::VecDeque;

fn video_stitching(mut clips: Vec<Vec<i32>>, time: i32) -> i32 {
    clips.sort_by_key(|a| a[0]);
    let mut queue = VecDeque::new();
    let n = clips.len();
    for i in 0..n {
        if clips[i][0] == 0 {
            queue.push_back((clips[i][0], clips[i][1], 1, i));
        }
    }

    while let Some((start, end, count, i)) = queue.pop_front() {
        if start == 0 && end >= time {
            return count;
        }

        for i in i+1..n {
            if clips[i][0] > end {
                break;
            }
            if clips[i][1] > end {
                queue.push_back((start, clips[i][1], count+1, i));
            }
        }
    }

    -1
}

pub fn main() {
    // let clips = [[0,2],[4,6],[8,10],[1,9],[1,5],[5,9]].map(Vec::from).into_iter().collect();
    // let time = 10;
    let clips = [[0,1],[6,8],[0,2],[5,6],[0,4],[0,3],[6,7],[1,3],[4,7],[1,4],[2,5],[2,6],[3,4],[4,5],[5,7],[6,9]].map(Vec::from).into_iter().collect();
    let time = 9;
    println!("{}", video_stitching(clips, time));
}
