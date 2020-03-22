use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let len1: usize = arr1.len();
        let len2: usize = arr2.len();
        let len: usize = if len1 > len2 { len1 + 2 } else { len2 + 2 };
        let mut result: Vec<i32> = vec![0; len];
        let mut i: usize = 1;
        while i <= len - 2 {
            let t: i32 = if i <= len1 { arr1[len1 - i] } else { 0 }
                + if i <= len2 { arr2[len2 - i] } else { 0 }
                + result[len - i];
            if t & 1 == 1 {
                result[len - i] = 1;
            } else {
                result[len - i] = 0;
            }
            i += 1;
            if t > 1 {
                // Carry: [1, 1]
                if result[len - i] == 1 {
                    // [1, 1] + [0, 1] = [0, 0]
                    // [1, 1] + [1, 1] = [1, 0]
                    result[len - i] = 0;
                } else {
                    // [1, 1] + [0, 0] = [1, 1]
                    result[len - i] = 1;
                    result[len - i - 1] = 1;
                }
            }
        }
        while result.len() > 1 && result[0] == 0 {
            result.remove(0);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut arr1: Vec<i32> = Vec::new();
    let mut arr2: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                if arr1.len() == n as usize {
                    arr2.push(number);
                } else {
                    arr1.push(number);
                }
            }
        }
    }

    if ret == 0 {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<i32> = Solution::add_negabinary(arr1, arr2);
    for r in result.iter() {
        print!("{} ", *r);
    }
    println!();
}
