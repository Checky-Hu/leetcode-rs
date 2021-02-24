use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points_mut = points;
        points_mut.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut distance: i32 = 0;
        let mut result: i32 = 0;
        for (i, point) in points_mut.iter().enumerate() {
            if i == 0 {
                distance = point[1];
                result += 1;
            } else if distance >= point[0] {
                if distance > point[1] {
                    distance = point[1];
                }
            } else {
                distance = point[1];
                result += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut points: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            tmp.push(n);
            if tmp.len() == 2 {
                points.push(tmp);
                tmp = Vec::new();
            }
        }
    }

    if 2 > ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!(
        "Minimum number of arrows: {}",
        Solution::find_min_arrow_shots(points)
    );
}
