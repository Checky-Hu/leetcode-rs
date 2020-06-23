use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let mut distance: Vec<Vec<i32>> = vec![vec![i32::max_value() / 2; n as usize]; n as usize];
        for edge in edges.iter() {
            distance[edge[0] as usize][edge[1] as usize] = edge[2];
            distance[edge[1] as usize][edge[0] as usize] = edge[2];
        }
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    let t: i32 =
                        distance[j as usize][i as usize] + distance[i as usize][k as usize];
                    if distance[j as usize][k as usize] > t {
                        distance[j as usize][k as usize] = t;
                    }
                }
            }
        }
        // (city, count)
        let mut result: (i32, i32) = (n, i32::max_value());
        for i in 0..n {
            let mut count: i32 = 0;
            for j in 0..n {
                if i == j {
                    continue;
                }
                if distance[i as usize][j as usize] <= distance_threshold {
                    count += 1;
                }
            }
            if count <= result.1 {
                result.0 = i;
                result.1 = count;
            }
        }
        result.0
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut distance_threshold: i32 = 0;
    let mut edges: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            2 => distance_threshold = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(t);
                if tmp.len() == 3 {
                    edges.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }

    println!(
        "The city: {}",
        Solution::find_the_city(n, edges, distance_threshold)
    );
}
