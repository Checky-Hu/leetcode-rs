use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let len: usize = graph.len();
        let mut colors: Vec<i32> = vec![0; len];
        for i in 0..len {
            if colors[i] != 0 {
                continue;
            }
            colors[i] = 1;
            let mut queue: Vec<usize> = vec![i];
            while !queue.is_empty() {
                let mut tmp: Vec<usize> = Vec::new();
                for q in queue {
                    for g in &graph[q] {
                        if colors[q] == colors[*g as usize] {
                            return false;
                        }
                        if colors[*g as usize] == 0 {
                            colors[*g as usize] = 0 - colors[q];
                            tmp.push(*g as usize);
                        }
                    }
                }
                queue = tmp;
            }
        }
        true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut graph: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    let mut count: i32 = -1;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                if count < 0 {
                    count = n;
                } else {
                    tmp.push(n);
                    if tmp.len() == count as usize {
                        graph.push(tmp);
                        tmp = Vec::new();
                        count = -1;
                    }
                }
            }
        }
    }

    if 0 == ret || count != -1 {
        println!("Require at least one parameter.");
        return;
    }

    println!("Bipartite: {}", Solution::is_bipartite(graph));
}
