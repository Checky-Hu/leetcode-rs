use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let a: f64 = (coordinates[0][1] - coordinates[1][1]) as f64
            / (coordinates[0][0] - coordinates[1][0]) as f64;
        let b: f64 = coordinates[0][1] as f64 - a * coordinates[0][0] as f64;
        for coordinate in coordinates.iter().skip(2) {
            if (coordinate[1] as f64 - a * coordinate[0] as f64 - b).abs() > std::f64::EPSILON {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut coordinates: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(n);
                if tmp.len() == 2 {
                    coordinates.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least four parameters.");
        return;
    }

    println!(
        "Is straight line: {}",
        Solution::check_straight_line(coordinates)
    );
}
