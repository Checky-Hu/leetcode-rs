use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn find_root(root: &[i32], t: i32) -> i32 {
        let mut i: i32 = t;
        while root[i as usize] != -1 {
            i = root[i as usize];
        }
        i
    }

    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(2);
        let mut root: Vec<i32> = vec![-1; 2001];
        for edge in &edges {
            let a: i32 = Solution::find_root(&root, edge[0]);
            let b: i32 = Solution::find_root(&root, edge[1]);
            if a == b {
                result.push(edge[0]);
                result.push(edge[1]);
                break;
            } else {
                root[a as usize] = b;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut edges: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::with_capacity(2);
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(n);
                if tmp.len() == 2 {
                    edges.push(tmp);
                    tmp = Vec::with_capacity(2);
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }

    let result: Vec<i32> = Solution::find_redundant_connection(edges);
    println!("Redundant connection: [{}, {}]", result[0], result[1]);
}
