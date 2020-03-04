use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut index: Vec<usize> = Vec::new();
        let mut distances: Vec<i32> = Vec::new();
        for (i, point) in points.iter().enumerate() {
            index.push(i);
            distances.push(point[0] * point[0] + point[1] * point[1]);
        }
        index.sort_by(|a, b| distances[*a].cmp(&distances[*b]));
        let mut result: Vec<Vec<i32>> = Vec::new();
        for i in 0..k {
            result.push(vec![points[index[i as usize]][0], points[index[i as usize]][1]]);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut points: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(number);
                if tmp_row.len() == 2 {
                    points.push(tmp_row);
                    tmp_row = Vec::new();
                }
            }
        }
    }

    if ret == 0 {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<Vec<i32>> = Solution::k_closest(points, k);
    for r in result.iter() {
        print!("[{}, {}] ", r[0], r[1]);
    }
    println!();
}
