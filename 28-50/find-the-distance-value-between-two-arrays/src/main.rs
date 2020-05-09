use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        let mut result: i32 = 0;
        for n1 in arr1.iter() {
            let mut is_distance: bool = true;
            for n2 in arr2.iter() {
                if (*n1 - *n2).abs() <= d {
                    is_distance = false;
                    break;
                }
            }
            if is_distance {
                result += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut d: i32 = 0;
    let mut len1: i32 = 0;
    let mut arr1: Vec<i32> = Vec::new();
    let mut arr2: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => d = i32::from_str(&arg).expect("Error parse."),
            2 => len1 = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                if ret > len1 {
                    arr2.push(n);
                } else {
                    arr1.push(n);
                }
            }
        }
    }

    if 0 == len1 || len1 >= ret {
        println!("Require at least (1 + arg1) parameters.");
        return;
    }

    println!(
        "Distance value: {}",
        Solution::find_the_distance_value(arr1, arr2, d)
    );
}
