use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_score_sightseeing_pair(a: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let mut index: usize = 0;
        for i in 1..a.len() {
            let t: i32 = a[index] + a[i] - (i - index) as i32;
            if t > result {
                result = t;
            }
            if a[index] - a[i] < (i - index) as i32 {
                index = i;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            a.push(n);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Max score sightseeing pair: {}",
        Solution::max_score_sightseeing_pair(a)
    );
}
