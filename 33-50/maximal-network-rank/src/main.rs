use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let u: usize = n as usize;
        let mut flags: Vec<Vec<bool>> = vec![vec![false; u]; u];
        let mut count: Vec<i32> = vec![0; u];
        for v in roads.iter() {
            flags[v[0] as usize][v[1] as usize] = true;
            flags[v[1] as usize][v[0] as usize] = true;
            count[v[0] as usize] += 1;
            count[v[1] as usize] += 1;
        }
        let mut result: i32 = 0;
        for i in 0..(u - 1) {
            for j in (i + 1)..u {
                let t: i32 = count[i] + count[j] + if flags[i][j] { -1 } else { 0 };
                if t > result {
                    result = t;
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut n: i32 = 0;
    let mut roads: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(num);
                if tmp.len() == 2 {
                    roads.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 2 > ret {
        println!("Require at least 3 parameters.");
        return;
    }

    println!(
        "Maximal network rank: {}",
        Solution::maximal_network_rank(n, roads)
    );
}
