use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut a: i32 = 0;
        let mut a_count: i32 = 0;
        let mut b: i32 = 0;
        let mut b_count: i32 = 0;
        for n in &nums {
            if *n == a {
                a_count += 1;
            } else if *n == b {
                b_count += 1;
	    } else {
                if a_count == 0 {
                    a = *n;
	            a_count = 1;
                } else if b_count == 0 {
                    b = *n;
	            b_count = 1;
                } else {
                    a_count -= 1;
                    b_count -= 1;
                }
            }
        }
        a_count = 0;
        b_count = 0;
        for n in &nums {
            if *n == a {
                a_count += 1;
            } else if *n == b {
                b_count += 1;
            }
        }
        let mut result: Vec<i32> = Vec::with_capacity(2);
        if a_count as usize > nums.len() / 3 {
            result.push(a);
        }
        if b_count as usize > nums.len() / 3 {
            result.push(b);
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
            nums.push(number);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<i32> = Solution::majority_element(nums);
    for i in result {
        print!("{} ", i);
    }
    print!("\n");
}
