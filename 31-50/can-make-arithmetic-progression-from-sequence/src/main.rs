use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut mut_arr: Vec<i32> = arr;
        mut_arr.sort();
        let delta: i32 = mut_arr[1] - mut_arr[0];
        for i in 2..mut_arr.len() {
            if delta != mut_arr[i] - mut_arr[i - 1] {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut arr: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(t);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!(
        "Can make arithmetic progression: {}",
        Solution::can_make_arithmetic_progression(arr)
    );
}
