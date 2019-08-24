extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let len: usize = costs.len();
        let mut result: i32 = 0;
        let mut diff: Vec<i32> = Vec::new();
        for v in costs {
            result += v[0];
            diff.push(v[0] - v[1]);
        }
        qsi32::quick_sort(&mut diff, 0, len - 1);
        let mut i: usize = len - 1;
        while i >= len / 2 {
            result -= diff[i];
            i -= 1;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut costs: Vec<Vec<i32>> = Vec::new();
    let mut tmp_row: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp_row.push(number);
                if tmp_row.len() == 2 {
                    costs.push(tmp_row);
                    tmp_row = Vec::new();
                }
            },
        }
    }

    if 0 == ret || 0 != ret & 1 {
        println!("Require at least (2 * n) parameters.");
        return;
    }

    println!("Cost: {}", Solution::two_city_sched_cost(costs));
}
