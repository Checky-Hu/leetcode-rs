use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        if edges[0][0] == edges[1][0] || edges[0][0] == edges[1][1] {
            edges[0][0]
        } else {
            edges[0][1]
        }
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut edge: Vec<i32> = Vec::new();
    let mut edges: Vec<Vec<i32>> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                edge.push(num);
                if edge.len() == 2 {
                    edges.push(edge);
                    edge = Vec::new();
                }
            }
        }
    }

    if 0 == ret || !edge.is_empty() {
        println!("Require at least even parameters.");
        return;
    }

    println!("Center of star graph: {}", Solution::find_center(edges));
}
