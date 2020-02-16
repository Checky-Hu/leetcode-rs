use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let mut queue: Vec<Vec<usize>> = Vec::new();
        let mut color: Vec<i32> = vec![0; n as usize];
        for (i, v) in dislikes.iter().enumerate() {
            if i == 0 {
                color[v[0] as usize - 1] = 1;
                color[v[1] as usize - 1] = -1;
            } else {
                if color[v[0] as usize - 1] == 0 {
                    if color[v[1] as usize - 1] == 0 {
                        queue.push(vec![v[0] as usize - 1, v[1] as usize - 1]);
                    } else {
                        color[v[0] as usize - 1] = 0 - color[v[1] as usize - 1];
                    }
                } else {
                    if color[v[1] as usize - 1] == 0 {
                        color[v[1] as usize - 1] = 0 - color[v[0] as usize - 1];
                    } else if color[v[0] as usize - 1] + color[v[1] as usize - 1] != 0 {
                        return false;
                    }
                }
            }
        }
        loop {
            let mut tmp: Vec<Vec<usize>> = Vec::new();
            let mut change_color: bool = false;
            for q in queue.iter() {
                if color[q[0]] == 0 {
                    if color[q[1]] == 0 {
                        tmp.push(vec![q[0], q[1]]);
                    } else {
                        change_color = true;
                        color[q[0]] = 0 - color[q[1]];
                    }
                } else {
                    if color[q[1]] == 0 {
                        change_color = true;
                        color[q[1]] = 0 - color[q[0]];
                    } else if color[q[0]] + color[q[1]] != 0 {
                        return false;
                    }
                }
            }
            if !change_color {
                if tmp.is_empty() {
                    break;
                } else {
                    color[tmp[0][0]] = 1;
                    color[tmp[0][1]] = -1;
                }
            }
            queue = tmp;
        }
        true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut dislikes: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(number);
                if tmp.len() == 2 {
                    dislikes.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least (1 + 2 * n) parameters.");
        return;
    }

    println!(
        "Possible to bipartition: {}",
        Solution::possible_bipartition(n, dislikes)
    );
}
