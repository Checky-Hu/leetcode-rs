use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        let mut h_cuts: Vec<i32> = horizontal_cuts;
        h_cuts.sort();
        let mut pre_v: i32 = 0;
        let mut max_h: i32 = 0;
        for cut in h_cuts.iter() {
            let t: i32 = *cut - pre_v;
            if t > max_h {
                max_h = t;
            }
            pre_v = *cut;
        }
        let t: i32 = h - pre_v;
        if t > max_h {
            max_h = t;
        }
        let mut w_cuts: Vec<i32> = vertical_cuts;
        w_cuts.sort();
        pre_v = 0;
        let mut max_w: i32 = 0;
        for cut in w_cuts.iter() {
            let t: i32 = *cut - pre_v;
            if t > max_w {
                max_w = t;
            }
            pre_v = *cut;
        }
        let t: i32 = w - pre_v;
        if t > max_w {
            max_w = t;
        }
        (max_h as i64 * max_w as i64 % 1_000_000_007) as i32
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut h: i32 = 0;
    let mut w: i32 = 0;
    let mut l: i32 = 0;
    let mut horizontal_cuts: Vec<i32> = Vec::new();
    let mut vertical_cuts: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => h = i32::from_str(&arg).expect("Error parse."),
            2 => w = i32::from_str(&arg).expect("Error parse."),
            3 => l = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                if horizontal_cuts.len() == l as usize {
                    vertical_cuts.push(number);
                } else {
                    horizontal_cuts.push(number);
                }
            }
        }
    }

    if 0 == h || 0 == w || 0 == ret {
        println!("Require at least (3 + arg3) parameters.");
        return;
    }

    println!(
        "Maximum area: {}",
        Solution::max_area(h, w, horizontal_cuts, vertical_cuts)
    );
}
