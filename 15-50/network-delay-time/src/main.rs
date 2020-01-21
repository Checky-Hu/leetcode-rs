use std::collections::HashSet;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let len: usize = 101;
        let mut edges: Vec<Vec<i32>> = vec![vec![-1; len]; len];
        let mut queue: Vec<usize> = vec![k as usize];
        let mut dist: Vec<i32> = vec![i32::max_value(); n as usize + 1];
        dist[k as usize] = 0;
        for time in &times {
            edges[time[0] as usize][time[1] as usize] = time[2];
        }
        while !queue.is_empty() {
            let mut tmp: Vec<usize> = Vec::new();
            let mut set: HashSet<usize> = HashSet::new();
            for src in &queue {
                for i in 1..len {
                    if edges[*src][i] != -1 && dist[*src] + edges[*src][i] < dist[i] {
                        if !set.contains(&i) {
                            set.insert(i);
                            tmp.push(i);
                        }
                        dist[i] = dist[*src] + edges[*src][i];
                    }
                }
            }
            queue = tmp;
        }
        let mut result: i32 = 0;
        for d in dist.iter().skip(1) {
            if *d > result {
                result = *d;
            }
        }
        if result == i32::max_value() {
            -1
        } else {
            result
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut k: i32 = 0;
    let mut times: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::with_capacity(2);
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            2 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(t);
                if tmp.len() == 3 {
                    times.push(tmp);
                    tmp = Vec::with_capacity(3);
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!(
        "Mas delay time: {}",
        Solution::network_delay_time(times, n, k)
    );
}
