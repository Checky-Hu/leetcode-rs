use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        if points[0][0] == points[1][0] {
            points[0][1] != points[1][1] && points[0][0] != points[2][0]
        } else if points[0][1] == points[1][1] {
            points[0][0] != points[1][0] && points[0][1] != points[2][1]
        } else {
            let k: f32 = (points[0][1] - points[1][1]) as f32 / (points[0][0] - points[1][0]) as f32;
            let b: f32 = points[0][1] as f32 - k * (points[0][0] as f32);
            points[2][1] as f32 != k * (points[2][0] as f32) + b
        }
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
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(number);
                if tmp_row.len() == 2 {
                    points.push(tmp_row);
                    if points.len() == 3 {
                        break;
                    } else {
                        tmp_row = Vec::new();
                    }
                }
            },
        }
    }

    if 6 != ret {
        println!("Require at least 6 parameters.");
        return;
    }

    println!("Boomerang: {}", Solution::is_boomerang(points));
}
