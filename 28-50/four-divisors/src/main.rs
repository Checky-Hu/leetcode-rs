use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn get_divisors(num: i32) -> i32 {
        let mut counts: i32 = 0;
        let mut result: Vec<i32> = Vec::with_capacity(4);
        let sqrt: i32 = f64::from(num).sqrt().trunc() as i32;
        for i in 1..=sqrt {
            if num % i == 0 {
                if counts == 2 {
                    return 0;
                } else if i * i != num {
                    result.push(i);
                    result.push(num / i);
                    counts += 1;
                }
            }
        }
        if counts == 2 {
            result[0] + result[1] + result[2] + result[3]
        } else {
            0
        }
    }

    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        for num in nums.iter() {
            result += Solution::get_divisors(*num);
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
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }

    println!(
        "Sum of four divisors: {}",
        Solution::sum_four_divisors(nums)
    );
}
