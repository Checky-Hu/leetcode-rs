use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let target: usize = arr.len() / 4;
        let mut count: usize = 0;
        let mut pre_i: i32 = -1;
        for n in &arr {
            if *n == pre_i {
                count += 1;
                if count > target {
                    return pre_i;
                }
            } else {
                pre_i = *n;
                count = 1;
            }
        }
        pre_i
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
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Special integer: {}", Solution::find_special_integer(arr));
}
