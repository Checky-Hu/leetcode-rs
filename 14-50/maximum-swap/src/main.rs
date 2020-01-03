use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        if num == 0 {
            return 0;
        }
        let mut nums: Vec<i32> = Vec::new();
        let mut t: i32 = num;
        while t > 0 {
            nums.push(t % 10);
            t /= 10;
        }
        let len: usize = nums.len();
        let mut i: usize = len - 1;
        while i > 0 {
            let mut max: i32 = nums[i];
            let mut pos: usize = i;
            for (j, v) in nums.iter().enumerate().take(i) {
                if *v > max {
                    max = *v;
                    pos = j;
                }
            }
            if max > nums[i] {
                nums.swap(pos, i);
                break;
            } else {
                i -= 1;
            }
        }
        let mut result: i32 = 0;
        i = len - 1;
        loop {
            result = result * 10 + nums[i];
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Maximum swap: {}", Solution::maximum_swap(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
