use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        // (color, node), color: 0 -> none, 1 -> red, 2 -> blue.
        let mut out: Vec<Vec<(u8, i32)>> = vec![Vec::new(); n as usize];
        for red_edge in red_edges.iter() {
            out[red_edge[0] as usize].push((1, red_edge[1]));
        }
        for blue_edge in blue_edges.iter() {
            out[blue_edge[0] as usize].push((2, blue_edge[1]));
        }
        let mut result: Vec<i32> = vec![-1; n as usize];
        result[0] = 0;
        // (visit in red, visit in blue)
        let mut visit: Vec<Vec<bool>> = vec![vec![false; 2]; n as usize];
        let mut queue: Vec<(u8, i32)> = vec![(0, 0)];
        let mut path: i32 = 1;
        while !queue.is_empty() {
            let mut temp: Vec<(u8, i32)> = Vec::new();
            for node in queue {
                for next in out[node.1 as usize].iter() {
                    if 0 == node.0 || node.0 != next.0 {
                        if !visit[next.1 as usize][next.0 as usize - 1] {
                            temp.push(*next);
                            visit[next.1 as usize][next.0 as usize - 1] = true;
                        }
                        if result[next.1 as usize] == -1 {
                            result[next.1 as usize] = path;
                        }
                    }
                }
            }
            queue = temp;
            path += 1;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut k: i32 = 0;
    let mut blue_edges: Vec<Vec<i32>> = Vec::new();
    let mut red_edges: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            2 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let node: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(node);
                if tmp_row.len() == 2 {
                    if red_edges.len() == k as usize {
                        blue_edges.push(tmp_row);
                    } else {
                        red_edges.push(tmp_row);
                    }
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    let result: Vec<i32> = Solution::shortest_alternating_paths(n, red_edges, blue_edges);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
