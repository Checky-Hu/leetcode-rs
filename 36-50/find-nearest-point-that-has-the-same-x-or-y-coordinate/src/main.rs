use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        // (distance, index)
        let mut result: (i32, i32) = (i32::MAX, -1);
        for (i, v) in points.iter().enumerate() {
            let t = if v[0] == x {
                (v[1] - y).abs()
            } else if v[1] == y {
                (v[0] - x).abs()
            } else {
                i32::MAX
            };
            if t < result.0 {
                result.0 = t;
                result.1 = i as i32;
            }
        }
        result.1
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut points: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => x = i32::from_str(&arg).expect("Error parse."),
            2 => y = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(n);
                if tmp.len() == 2 {
                    points.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 2 > ret {
        println!("Require at least 4 parameters.");
        return;
    }

    println!(
        "Nearest valid point: {}",
        Solution::nearest_valid_point(x, y, points)
    );
}
