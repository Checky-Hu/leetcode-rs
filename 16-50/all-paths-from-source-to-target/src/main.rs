use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n: i32 = graph.len() as i32 - 1;
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut queue: Vec<Vec<i32>> = vec![vec![0]];
        while !queue.is_empty() {
            let mut next: Vec<Vec<i32>> = Vec::new();
            while let Some(x) = queue.pop() {
                if let Some(y) = x.last() {
                    for t in &graph[*y as usize] {
                        let mut tmp: Vec<i32> = x.clone();
                        tmp.push(*t);
                        if *t == n {
                            result.push(tmp);
                        } else {
                            next.push(tmp);
                        }
                    }
                }
            }
            queue = next;
        }
        result
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
                    if n == 0 {
                        graph.push(tmp);
                        tmp = Vec::new();
                    } else {
                        count = n;
                    }
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

    let result: Vec<Vec<i32>> = Solution::all_paths_source_target(graph);
    for r in &result {
        for n in r {
            print!("{} ", *n);
        }
        println!();
    }
}
