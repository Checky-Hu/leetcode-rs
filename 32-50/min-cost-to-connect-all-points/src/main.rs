use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let len: usize = points.len();
        let mut visits: Vec<bool> = vec![false; len];
        visits[0] = true;
        let mut distances: Vec<i32> = vec![i32::max_value(); len];
        let mut current: usize = 0;
        let mut result: i32 = 0;
        for _i in 1..len {
            let mut min_distance: i32 = i32::max_value();
            let mut min_index: usize = 0;
            for (i, v) in visits.iter().enumerate() {
                if !*v {
                    let t: i32 = (points[i][0] - points[current][0]).abs()
                        + (points[i][1] - points[current][1]).abs();
                    if distances[i] > t {
                        distances[i] = t;
                    }
                    if min_distance > distances[i] {
                        min_distance = distances[i];
                        min_index = i;
                    }
                }
            }
            result += min_distance;
            current = min_index;
            visits[current] = true;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut points: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(t);
                if tmp_row.len() == 2 {
                    points.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if 0 == ret || !tmp_row.is_empty() {
        println!("Require at least 2 parameters.");
        return;
    }

    println!(
        "Minimum cost to connect all points: {}",
        Solution::min_cost_connect_points(points)
    );
}
