use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        // (max, count)
        let mut result: (i32, i32) = (0, 0);
        for v in rectangles.iter() {
            let len: i32 = if v[0] > v[1] { v[1] } else { v[0] };
            match result.0.cmp(&len) {
                Ordering::Greater => (),
                Ordering::Equal => result.1 += 1,
                Ordering::Less => {
                    result.0 = len;
                    result.1 = 1;
                }
            }
        }
        result.1
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut tmp: Vec<i32> = Vec::new();
    let mut rectangles: Vec<Vec<i32>> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(n);
                if tmp.len() == 2 {
                    rectangles.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 2 > ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!(
        "Number of good rectangles: {}",
        Solution::count_good_rectangles(rectangles)
    );
}
