use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn eventual_safe_node(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let len: usize = graph.len();
        let mut paths: Vec<usize> = vec![0; len];
        let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut queue: Vec<usize> = Vec::new();
        for (i, v) in graph.iter().enumerate() {
            if v.is_empty() {
                queue.push(i);
            } else {
                paths[i] = v.len();
                for dst in v.iter() {
                    match map.get_mut(&(*dst as usize)) {
                        Some(x) => x.push(i),
                        None => {
                            map.insert(*dst as usize, vec![i]);
                        }
                    }
                }
            }
        }
        while !queue.is_empty() {
            let mut tmp: Vec<usize> = Vec::new();
            for dst in queue {
                if let Some(x) = map.get(&dst) {
                    for src in x {
                        paths[*src] -= 1;
                        if paths[*src] == 0 {
                            tmp.push(*src);
                        }
                    }
                }
            }
            queue = tmp;
        }
        let mut result: Vec<i32> = Vec::new();
        for (i, v) in paths.iter().enumerate() {
            if *v == 0 {
                result.push(i as i32);
            }
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

    let result: Vec<i32> = Solution::eventual_safe_node(graph);
    for r in &result {
        print!("{} ", *r);
    }
    println!();
}
