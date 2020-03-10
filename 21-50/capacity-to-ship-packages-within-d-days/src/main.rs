use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn is_valid(capacity: i32, weights: &[i32], d: i32) -> bool {
        let mut count: i32 = 1;
        let mut cur_w: i32 = 0;
        for w in weights.iter() {
            cur_w += *w;
            match cur_w.cmp(&capacity) {
                Ordering::Equal => {
                    count += 1;
                    cur_w = 0;
                }
                Ordering::Greater => {
                    count += 1;
                    cur_w = *w;
                }
                Ordering::Less => (),
            }
        }
        count <= d
    }

    pub fn ship_within_days(weights: Vec<i32>, d: i32) -> i32 {
        let mut max: i32 = 0;
        let mut sum: i32 = 0;
        for w in weights.iter() {
            sum += *w;
            if *w > max {
                max = *w;
            }
        }
        let s: &mut i32 = &mut max;
        let e: &mut i32 = &mut sum;
        while *s < *e {
            let mid: i32 = *s + (*e - *s) / 2;
            if Solution::is_valid(mid, &weights, d) {
                *e = mid;
            } else {
                *s = mid + 1;
            }
        }
        *s
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut d: i32 = 0;
    let mut weights: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => d = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                weights.push(number);
            }
        }
    }

    if ret == 0 {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Least weight capacity: {}",
        Solution::ship_within_days(weights, d)
    );
}
