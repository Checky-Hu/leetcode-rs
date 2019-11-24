use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let len: usize = nums.len();
        if len == 0 {
            return Vec::new()
        }
        let mut result: Vec<i32> = vec![-1; len];
        let mut s: Vec<usize> = Vec::with_capacity(len);
        for i in 0..(2 * len) {
            let tmp: i32 = nums[i % len];
            loop {
                match s.last() {
                    Some(x) => {
                        if nums[*x] < tmp {
                            result[*x] = tmp;
                            s.pop();
                        } else {
                            break;
                        }
                    },
                    None => break,
                }
            }
            if i < len {
                s.push(i);
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(num);
            },
        }
    }

    if 0 == ret {
        println!("Require at least one parameters.");
        return
    }

    let result: Vec<i32> = Solution::next_greater_elements(nums);
    for n in result {
        print!("{} ", n);
    }
    print!("\n");
}
