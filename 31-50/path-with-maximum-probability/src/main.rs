use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_probability(
        n: i32,
        edges: Vec<Vec<i32>>,
        succ_prob: Vec<f64>,
        start: i32,
        end: i32,
    ) -> f64 {
        let mut map: HashMap<i32, HashMap<i32, f64>> = HashMap::new();
        for (i, edge) in edges.iter().enumerate() {
            match map.get_mut(&edge[0]) {
                Some(x) => {
                    x.insert(edge[1], succ_prob[i]);
                }
                None => {
                    let mut new: HashMap<i32, f64> = HashMap::new();
                    new.insert(edge[1], succ_prob[i]);
                    map.insert(edge[0], new);
                }
            }
            match map.get_mut(&edge[1]) {
                Some(x) => {
                    x.insert(edge[0], succ_prob[i]);
                }
                None => {
                    let mut new: HashMap<i32, f64> = HashMap::new();
                    new.insert(edge[0], succ_prob[i]);
                    map.insert(edge[1], new);
                }
            }
        }
        let mut visit: Vec<f64> = vec![-1_f64; n as usize];
        visit[start as usize] = 1_f64;
        let mut queue: Vec<i32> = Vec::with_capacity(n as usize);
        queue.push(start);
        let mut result: f64 = 0_f64;
        while !queue.is_empty() {
            let mut tmp: Vec<i32> = Vec::new();
            for q in queue {
                if let Some(x) = map.get(&q) {
                    for (k, v) in x.iter() {
                        let cur: f64 = visit[q as usize] * *v;
                        if visit[*k as usize] < cur {
                            visit[*k as usize] = cur;
                            tmp.push(*k);
                        }
                        if *k == end && cur > result {
                            result = cur;
                        }
                    }
                }
            }
            queue = tmp;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut start: i32 = 0;
    let mut end: i32 = 0;
    let mut count: i32 = 0;
    let mut edges: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    let mut succ_prob: Vec<f64> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            2 => start = i32::from_str(&arg).expect("Error parse."),
            3 => end = i32::from_str(&arg).expect("Error parse."),
            4 => count = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                if edges.len() == count as usize {
                    let t: f64 = f64::from_str(&arg).expect("Error parse.");
                    succ_prob.push(t);
                } else {
                    let t: i32 = i32::from_str(&arg).expect("Error parse.");
                    tmp_row.push(t);
                    if tmp_row.len() == 2 {
                        edges.push(tmp_row);
                        tmp_row = Vec::new();
                    }
                }
            }
        }
    }

    if 0 == n || 0 == count || 3 * count != ret {
        println!("Require at least (4 + arg4 * 3) parameters.");
        return;
    }

    println!(
        "Maximum probability: {}",
        Solution::max_probability(n, edges, succ_prob, start, end)
    );
}
