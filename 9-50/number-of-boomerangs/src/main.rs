use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut result: i32 = 0;
        for cur in &points {
            let mut tmp: HashMap<i32, i32> = HashMap::new();
            for p in &points {
                let x: i32 = p[0] - cur[0];
                let y: i32 = p[1] - cur[1];
                let d: i32 = x * x + y * y;
                match tmp.get_mut(&d) {
                    Some(x) => *x += 1,
                    None => {
                        tmp.insert(d, 1);
                    },
                }
            }
            for v in tmp.values() {
                result += v * (v - 1);
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
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(number);
                if tmp.len() == 2 {
                    points.push(tmp);
                    tmp = Vec::new();
                }
            },
        }
    }

    if 0 == ret || ret & 1 == 1 {
        println!("Require at least (2 * n) parameters.");
        return;
    }

    println!("Number of boomerangs: {}", Solution::number_of_boomerangs(points));
}
