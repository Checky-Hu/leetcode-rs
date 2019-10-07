use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let len: usize = primes.len();
        let mut pos: Vec<usize> = vec![0; len];
        let mut tmp: Vec<i32> = vec![0; len];
        let mut nums: Vec<i32> = vec![1];
        let mut index: usize = 1;
        while index < n as usize {
            let mut min: i32 = i32::max_value();
            for i in 0..len {
                tmp[i] = nums[pos[i]] * primes[i];
                if tmp[i] < min {
                    min = tmp[i];
                }
            }
            nums.push(min);
            for i in 0..len {
                if tmp[i] == min {
                    pos[i] += 1;
                }
            }
            index += 1;
        }
        nums[n as usize - 1]
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut n: i32 = 0;
    let mut primes: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let prime: i32 = i32::from_str(&arg).expect("Error parse.");
                primes.push(prime);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return
    }

    println!("Number: {}", Solution::nth_super_ugly_number(n, primes));
}
