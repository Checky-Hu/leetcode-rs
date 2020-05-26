use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn subtract_to_next(k: i32, vec: &[i32], index: usize) -> i32 {
        let mut result: i32 = -1;
        let mut i: usize = index;
        loop {
            if vec[i] == k {
                result = 1;
                break;
            } else if i == 0 {
                break;
            } else if vec[i] < k {
                let t: i32 = Solution::subtract_to_next(k - vec[i], vec, i - 1);
                if t > 0 {
                    result = t + 1;
                    break;
                }
            }
            i -= 1;
        }
        result
    }

    pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
        let mut fibonacci: Vec<i32> = Vec::new();
        let mut n1: i32 = 1;
        let mut n2: i32 = 1;
        fibonacci.push(1);
        fibonacci.push(1);
        loop {
            let n: i32 = n1 + n2;
            if n > k {
                break;
            } else {
                fibonacci.push(n);
                n1 = n2;
                n2 = n;
            }
        }
        Solution::subtract_to_next(k, &fibonacci, fibonacci.len() - 1)
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let k: i32 = i32::from_str(&arg).expect("Error parse.");
            println!(
                "Number of fibonacci: {}",
                Solution::find_min_fibonacci_numbers(k)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
