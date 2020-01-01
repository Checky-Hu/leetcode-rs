use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::with_capacity(k as usize);
        let len: usize = arr.len();
        let mut pos: usize = 0;
        for n in &arr {
            if *n >= x {
                break;
            } else {
                pos += 1;
            }
        }
        if pos == 0 {
            for v in arr.iter().take(k as usize) {
                result.push(*v);
            }
        } else if pos == len {
            for v in arr.iter().take(len).skip(len - k as usize) {
                result.push(*v);
            }
        } else {
            let mut left: usize = pos - 1;
            let mut right: usize = pos;
            while result.len() < k as usize {
                let use_front: bool = if left == len {
                    false
                } else if right == len {
                    true
                } else {
                    x - arr[left] <= arr[right] - x
                };
                if use_front {
                    result.insert(0, arr[left]);
                    if left == 0 {
                        left = len;
                    } else {
                        left -= 1;
                    }
                } else {
                    result.push(arr[right]);
                    if right != len {
                        right += 1;
                    }
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut x: i32 = 0;
    let mut arr: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            2 => x = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least three parameters.");
        return;
    }

    let result: Vec<i32> = Solution::find_closest_elements(arr, k, x);
    for r in &result {
        print!("{} ", *r);
    }
    println!();
}
