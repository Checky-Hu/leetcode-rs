use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut result: i32 = 0;
        let mut pre_x: i32 = points[0][0];
        let mut pre_y: i32 = points[0][1];
        for point in points.iter().skip(1) {
            let delta_x: i32 = (point[0] - pre_x).abs();
            let delta_y: i32 = (point[1] - pre_y).abs();
            result += if delta_x > delta_y { delta_x } else { delta_y };
            pre_x = point[0];
            pre_y = point[1];
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

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!(
        "Minimum time: {}",
        Solution::min_time_to_visit_all_points(points)
    );
}
