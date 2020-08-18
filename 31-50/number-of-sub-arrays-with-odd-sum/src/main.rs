use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut counts: Vec<i32> = vec![1, 0];
        let mut cur: usize = 0;
        let mut result: i32 = 0;
        for num in arr.iter() {
            cur ^= *num as usize & 1;
            result = (result + counts[1 - cur]) % 1_000_000_007;
            counts[cur] += 1;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut arr: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(t);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!(
        "Number of sub-arrays with odd sum: {}",
        Solution::num_of_subarrays(arr)
    );
}
