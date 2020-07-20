use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        // (connected, right_direction)
        let mut status: Vec<Vec<(i32, bool)>> = vec![Vec::new(); n as usize];
        for connection in connections.iter() {
            status[connection[0] as usize].push((connection[1], true));
            status[connection[1] as usize].push((connection[0], false));
        }
        let mut result: i32 = 0;
        let mut queue: Vec<i32> = vec![0];
        let mut visit: Vec<bool> = vec![false; n as usize];
        visit[0] = true;
        while !queue.is_empty() {
            let mut nexts: Vec<i32> = Vec::new();
            for q in queue {
                for v in status[q as usize].iter() {
                    if !visit[v.0 as usize] {
                        if v.1 {
                            result += 1;
                        }
                        nexts.push(v.0);
                        visit[v.0 as usize] = true;
                    }
                }
            }
            queue = nexts;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut connections: Vec<Vec<i32>> = Vec::new();
    let mut tmp_v: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_v.push(number);
                if tmp_v.len() == 2 {
                    connections.push(tmp_v);
                    tmp_v = Vec::new();
                }
            }
        }
    }

    if 0 == n || (n - 1) * 2 != ret {
        println!("Require at least [1 + (arg1 - 1) * 2] parameters.");
        return;
    }

    println!(
        "Minimum reorders: {}",
        Solution::min_reorder(n, connections)
    );
}
