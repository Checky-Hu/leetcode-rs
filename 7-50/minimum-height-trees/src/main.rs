use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }
        let mut len: usize = n as usize;
        let mut inputs: Vec<Vec<usize>> = vec![Vec::new(); len];
        let mut counts: Vec<i32> = vec![0; len];
        for edge in edges.iter() {
            let first: usize = edge[0] as usize;
            let second: usize = edge[1] as usize;
            inputs[first].push(second);
            inputs[second].push(first);
            counts[first] += 1;
            counts[second] += 1;
        }
        let mut result: Vec<i32> = Vec::new();
        let mut visits: Vec<bool> = vec![false; len];
        for (i, v) in counts.iter().enumerate() {
            if *v == 1 {
                result.push(i as i32);
                visits[i] = true;
            }
        }
        while len > 2 {
            len -= result.len();
            let mut next: Vec<i32> = Vec::new();
            for index in result.iter() {
                for v in inputs[*index as usize].iter() {
                    if !visits[*v] {
                        counts[*v] -= 1;
                        if counts[*v] == 0 || counts[*v] == 1 {
                            next.push(*v as i32);
                            visits[*v] = true;
                        }
                    }
                }
            }
            result = next;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut edges: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let point: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(point);
                if tmp.len() == 2 {
                    edges.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == n || (n - 1) * 2 != ret {
        println!("Require at least (arg1 * 2 - 1) parameters.");
        return;
    }

    let result = Solution::find_min_height_trees(n, edges);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
