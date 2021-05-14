use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        let mut mut_arr: Vec<i32> = arr;
        mut_arr.sort_unstable();
        let mut result: i32 = 0;
        for a in mut_arr.iter() {
            if *a > result {
                result += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut arr: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(num);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!(
        "Maximum element after decrementing and rearranging: {}",
        Solution::maximum_element_after_decrementing_and_rearranging(arr)
    );
}
