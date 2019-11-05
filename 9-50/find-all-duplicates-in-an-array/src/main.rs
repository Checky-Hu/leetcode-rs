use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut tmp: Vec<i32> = nums.clone();
        for i in 0..tmp.len() {
            let index: i32 = if tmp[i] > 0 {
                tmp[i] - 1
            } else {
                -1 - tmp[i]
            };
            if tmp[index as usize] < 0 {
                result.push(index + 1);
            }
            tmp[index as usize] = 0 - tmp[index as usize];
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret = index;
            let num: i32 = i32::from_str(&arg).expect("Error parse.");
            nums.push(num);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<i32> = Solution::find_duplicates(nums);
    for n in result {
        print!("{} ", n);
    }
    print!("\n");
}
