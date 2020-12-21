use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut sum: i32 = 0;
        for v in nums.iter() {
            sum += *v;
        }
        let len: usize = nums.len();
        match sum.cmp(&x) {
            Ordering::Less => -1,
            Ordering::Equal => len as i32,
            Ordering::Greater => {
                let target: i32 = sum - x;
                let mut result: usize = 0;
                let mut s: usize = 0;
                let mut e: usize = 0;
                let mut cur: i32 = 0;
                while e < len {
                    cur += nums[e];
                    match cur.cmp(&target) {
                        Ordering::Less => e += 1,
                        Ordering::Equal => {
                            let t: usize = e - s + 1;
                            if t > result {
                                result = t;
                            }
                            cur -= nums[s];
                            s += 1;
                            e += 1;
                        }
                        Ordering::Greater => {
                            cur = cur - nums[s] - nums[e];
                            s += 1;
                        }
                    }
                }
                if result == 0 {
                    -1
                } else {
                    (len - result) as i32
                }
            }
        }
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut x: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => x = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!("Minimum operations: {}", Solution::min_operations(nums, x));
}
