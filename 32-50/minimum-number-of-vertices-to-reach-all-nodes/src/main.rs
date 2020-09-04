use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut visits: Vec<bool> = vec![false; n as usize];
        for edge in edges.iter() {
            visits[edge[1] as usize] = true;
        }
        let mut result: Vec<i32> = Vec::new();
        for (i, v) in visits.iter().enumerate() {
            if !*v {
                result.push(i as i32);
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut edges: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(t);
                if tmp_row.len() == 2 {
                    edges.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if 0 == ret || !tmp_row.is_empty() {
        println!("Require at least (1 + 2 * n) parameters.");
        return;
    }

    let result: Vec<i32> = Solution::find_smallest_set_of_vertices(n, edges);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
