use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let len: usize = intervals.len();
        let mut result: Vec<Vec<i32>> = Vec::new();
        for i in 0..len {
            let mut is_covered: bool = false;
            for j in 0..len {
                if i != j
                    && intervals[i][0] >= intervals[j][0]
                    && intervals[i][1] <= intervals[j][1]
                {
                    is_covered = true;
                    break;
                }
            }
            if !is_covered {
                result.push(vec![intervals[i][0], intervals[i][1]]);
            }
        }
        result.len() as i32
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut intervals: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(t);
                if tmp.len() == 2 {
                    intervals.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!(
        "Non-covered intervals: {}",
        Solution::remove_covered_intervals(intervals)
    );
}
