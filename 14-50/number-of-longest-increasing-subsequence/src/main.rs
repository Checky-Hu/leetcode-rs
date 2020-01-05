use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let mut max: i32 = 0;
        let len: usize = nums.len();
        let mut count: Vec<i32> = vec![1; len];
        let mut length: Vec<i32> = vec![1; len];
        for i in 0..len {
            for j in 0..i {
                if nums[i] <= nums[j] {
                    continue;
                } else {
                    match length[i].cmp(&(length[j] + 1)) {
                        Ordering::Equal => count[i] += count[j],
                        Ordering::Less => {
                            length[i] = length[j] + 1;
                            count[i] = count[j];
                        }
                        Ordering::Greater => (),
                    }
                }
            }
            match max.cmp(&length[i]) {
                Ordering::Equal => result += count[i],
                Ordering::Less => {
                    max = length[i];
                    result = count[i];
                }
                Ordering::Greater => (),
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
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(number);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Number of lis: {}", Solution::find_number_of_lis(nums));
}
