use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let len: usize = height.len();
        if len == 0 {
            return 0;
        }
        let mut prefix: i32 = 0;
        let mut left: Vec<i32> = Vec::with_capacity(len);
        for h in height.iter() {
            if *h > prefix {
                prefix = *h;
            }
            left.push(prefix);
        }
        prefix = 0;
        let mut right: Vec<i32> = vec![0; len];
        let mut index: usize = len - 1;
        loop {
            if height[index] > prefix {
                prefix = height[index];
            }
            right[index] = prefix;
            if index == 0 {
                break;
            } else {
                index -= 1;
            }
        }
        let mut result: i32 = 0;
        for i in 0..len {
            result += if left[i] > right[i] {
                right[i]
            } else {
                left[i]
            } - height[i];
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut height: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let h: i32 = i32::from_str(&arg).expect("Error parse.");
            height.push(h);
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!("Trap rain: {}", Solution::trap(height));
}
