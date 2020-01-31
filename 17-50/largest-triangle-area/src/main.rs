use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut result: f64 = 0f64;
        let len: usize = points.len();
        for i in 0..(len - 2) {
            for j in (i + 1)..(len - 1) {
                for k in (j + 1)..len {
                    let x1: i32 = points[i][0];
                    let y1: i32 = points[i][1];
                    let x2: i32 = points[j][0];
                    let y2: i32 = points[j][1];
                    let x3: i32 = points[k][0];
                    let y3: i32 = points[k][1];
                    let tmp: f64 = (y1 * x2 + y2 * x3 + y3 * x1 - y1 * x3 - y2 * x1 - y3 * x2).abs()
                        as f64
                        / 2f64;
                    if tmp > result {
                        result = tmp;
                    }
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut points: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
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

    if 6 > ret {
        println!("Require at least six parameters.");
        return;
    }

    println!(
        "Largest triangle area: {}",
        Solution::largest_triangle_area(points)
    );
}
