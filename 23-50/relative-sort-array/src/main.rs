extern crate quicksort;

use std::collections::HashMap;
use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut flag: HashMap<i32, i32> = HashMap::new();
        for n in &arr2 {
            flag.insert(*n, 0);
        }

        let len: usize = arr1.len();
        let mut result: Vec<i32> = vec![0; len];
        let mut i: usize = len - 1;
        for n in &arr1 {
            if flag.contains_key(&n) {
                if let Some(x) = flag.get_mut(&n) {
                    *x += 1;
                }
            } else {
                result[i] = *n;
                i -= 1;
            }
        }

        let mut j: usize = 0;
        for n in &arr2 {
            let mut k: usize = 0;
            while k < *flag.get(&n).unwrap() as usize {
                result[j] = *n;
                k += 1;
                j += 1;
            }
        }
        qsi32::quick_sort(&mut result, i + 1, len - 1);
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut arr1_len: i32 = 0;
    let mut arr1: Vec<i32> = Vec::new();
    let mut arr2: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => arr1_len = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                if ret <= arr1_len {
                    arr1.push(n);
                } else {
                    arr2.push(n);
                }
            },
        }
    }

    if 0 == ret || ret <= arr1_len {
        println!("Require at least (arr1_len + 1) parameters.");
        return;
    }

    let result: Vec<i32> = Solution::relative_sort_array(arr1, arr2);
    for n in result {
        print!("{} ", n);
    }
    print!("\n");
}
