use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn spiral_matrix_iii(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let deltas: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        // action.0 means direction: 0 - east, 1 - south, 2 - west, 3 - north
        // action.1 means distance
        let mut action: (usize, i32) = (0, 1);
        let mut position: (i32, i32) = (r0, c0);
        let target: usize = (r * c) as usize;
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(target);
        result.push(vec![r0, c0]);
        loop {
            for _i in 0..action.1 {
                position.0 += deltas[action.0].0;
                position.1 += deltas[action.0].1;
                if 0 <= position.0 && position.0 < r && 0 <= position.1 && position.1 < c {
                    result.push(vec![position.0, position.1]);
                }
            }
            action.0 = (action.0 + 1) % 4;
            for _i in 0..action.1 {
                position.0 += deltas[action.0].0;
                position.1 += deltas[action.0].1;
                if 0 <= position.0 && position.0 < r && 0 <= position.1 && position.1 < c {
                    result.push(vec![position.0, position.1]);
                }
            }
            action.0 = (action.0 + 1) % 4;
            action.1 += 1;
            if result.len() == target {
                break;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut r: i32 = 0;
    let mut c: i32 = 0;
    let mut r0: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => r = i32::from_str(&arg).expect("Error parse."),
            2 => c = i32::from_str(&arg).expect("Error parse."),
            3 => r0 = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let c0: i32 = i32::from_str(&arg).expect("Error parse.");
                let result: Vec<Vec<i32>> = Solution::spiral_matrix_iii(r, c, r0, c0);
                for v in result {
                    print!("({}, {}) ", v[0], v[1]);
                }
                println!();
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least four parameters.");
    }
}
