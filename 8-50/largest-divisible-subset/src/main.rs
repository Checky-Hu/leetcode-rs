extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let len: usize = nums.len();
        if len == 0 {
            return Vec::new()
        }
        let mut tmp: Vec<i32> = nums.clone();
        qsi32::quick_sort(&mut tmp, 0, len - 1);

        let mut flags: Vec<i32> = vec![1; len];
        let mut parent: Vec<usize> = vec![0; len];
        let mut max_v: i32 = 1;
        let mut max_i: usize = 0;
        let mut i: usize = len - 1;
        loop {
            for j in (i + 1)..len {
                if tmp[j] % tmp[i] == 0 && flags[i] < flags[j] + 1 {
                    flags[i] = flags[j] + 1;
                    parent[i] = j;
                    if max_v < flags[i] {
                        max_v = flags[i];
                        max_i = i;
                    }
                }
            }
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }

        let mut result: Vec<i32> = Vec::new();
        for _i in 0..max_v {
            result.push(tmp[max_i]);
            max_i = parent[max_i];
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(num);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return
    }

    let result: Vec<i32> = Solution::largest_divisible_subset(nums);
    for r in result {
        print!("{} ", r);
    }
    print!("\n");
}
