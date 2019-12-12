use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        let len: usize = nums.len();
        match len {
            1 => nums[0].to_string(),
            2 => nums[0].to_string() + "/" + &nums[1].to_string(),
            _ => {
                let mut result: String = String::new();
                for i in 0..len {
                    result.push_str(&nums[i].to_string());
                    if i != len - 1 {
                        result.push('/');
                    }
                    if i == 0 {
                        result.push('(');
                    }
                }
                result.push(')');
                result
            },
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 < index {
            ret += 1;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
            nums.push(number);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return
    }

    println!("Optimal division string: {}", Solution::optimal_division(nums));
}
