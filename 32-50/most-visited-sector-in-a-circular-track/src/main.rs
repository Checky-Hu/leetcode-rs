use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let mut visited: Vec<i32> = vec![0; n as usize + 1];
        let len: usize = rounds.len();
        for i in 1..len {
            if rounds[i] >= rounds[i - 1] {
                for j in rounds[i - 1]..rounds[i] {
                    visited[j as usize] += 1;
                }
            } else {
                for j in rounds[i - 1]..=n {
                    visited[j as usize] += 1;
                }
                for j in 1..rounds[i] {
                    visited[j as usize] += 1;
                }
            }
        }
        visited[rounds[len - 1] as usize] += 1;
        let mut max: i32 = 0;
        let mut result: Vec<i32> = Vec::new();
        for (i, v) in visited.iter().enumerate() {
            match v.cmp(&max) {
                Ordering::Greater => {
                    max = *v;
                    result.clear();
                    result.push(i as i32);
                }
                Ordering::Equal => {
                    result.push(i as i32);
                }
                _ => (),
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut rounds: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let round: i32 = i32::from_str(&arg).expect("Error parse.");
                rounds.push(round);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    let result: Vec<i32> = Solution::most_visited(n, rounds);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
