use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn check_if_prerequisite(
        n: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let len: usize = n as usize;
        let mut vec: Vec<Vec<usize>> = vec![Vec::new(); len];
        for prerequisite in prerequisites.iter() {
            vec[prerequisite[0] as usize].push(prerequisite[1] as usize);
        }
        let mut reachable: Vec<Vec<bool>> = vec![vec![false; len]; len];
        for (i, v) in reachable.iter_mut().enumerate().take(len) {
            v[i] = true;
            let mut queue: Vec<usize> = vec![i];
            while !queue.is_empty() {
                let mut nexts: Vec<usize> = Vec::new();
                for q in queue {
                    for t in vec[q].iter() {
                        if !v[*t] {
                            v[*t] = true;
                            nexts.push(*t);
                        }
                    }
                }
                queue = nexts;
            }
        }
        let mut result: Vec<bool> = Vec::new();
        for query in queries.iter() {
            result.push(reachable[query[0] as usize][query[1] as usize]);
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut n: i32 = 0;
    let mut l: i32 = 0;
    let mut prerequisites: Vec<Vec<i32>> = Vec::new();
    let mut queries: Vec<Vec<i32>> = Vec::new();
    let mut tmp_v: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            2 => l = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_v.push(number);
                if tmp_v.len() == 2 {
                    if prerequisites.len() == l as usize {
                        queries.push(tmp_v);
                    } else {
                        prerequisites.push(tmp_v);
                    }
                    tmp_v = Vec::new();
                }
            }
        }
    }

    if 0 == n || ret & 1 == 1 {
        println!("Require at least (2 + arg2 * 2) parameters.");
        return;
    }

    let result: Vec<bool> = Solution::check_if_prerequisite(n, prerequisites, queries);
    for i in result {
        print!("{} ", i);
    }
    println!();
}
