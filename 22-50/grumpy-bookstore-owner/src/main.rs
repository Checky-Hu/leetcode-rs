use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
        let len: usize = customers.len();
        let mut sum: i32 = 0;
        for i in 0..len {
            if i < x as usize || grumpy[i] == 0 {
                sum += customers[i];
            }
        }
        let mut result: i32 = sum;
        for i in (x as usize)..len {
            if grumpy[i] == 1 {
                sum += customers[i];
            }
            if grumpy[i - x as usize] == 1 {
                sum -= customers[i - x as usize];
            }
            if sum > result {
                result = sum;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut x: i32 = 0;
    let mut n: i32 = 0;
    let mut customers: Vec<i32> = Vec::new();
    let mut grumpy: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => x = i32::from_str(&arg).expect("Error parse."),
            2 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                if customers.len() == n as usize {
                    grumpy.push(number);
                } else {
                    customers.push(number);
                }
            }
        }
    }

    if ret == 0 {
        println!("Require at least (2 + 2 * arg2) parameters.");
        return;
    }

    println!(
        "Maximum satisfied: {}",
        Solution::max_satisfied(customers, grumpy, x)
    );
}
