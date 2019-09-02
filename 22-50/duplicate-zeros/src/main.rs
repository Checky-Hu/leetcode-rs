use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let tmp: Vec<i32> = arr.clone();
        let len: usize = tmp.len();
        let mut arr_i: usize = 0;
        let mut tmp_i: usize = 0;
        while arr_i < len {
            if tmp[tmp_i] == 0 {
                arr[arr_i] = 0;
                if arr_i + 1 < len {
                    arr[arr_i + 1] = 0;
                }
                arr_i += 2;
            } else {
                arr[arr_i] = tmp[tmp_i];
                arr_i += 1;
            }
            tmp_i += 1;
        }
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
            },
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    Solution::duplicate_zeros(&mut arr);
    for n in arr {
        print!("{} ", n);
    }
    print!("\n");
}

