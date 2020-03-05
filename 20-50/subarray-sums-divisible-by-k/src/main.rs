use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn subarrays_div_by_k(a: Vec<i32>, k: i32) -> i32 {
        let len: usize = a.len();
        let mut sum: Vec<i32> = vec![0; len + 1];
        for i in 0..len {
            sum[i + 1] = sum[i] + a[i];
        }
        let mut count: Vec<i32> = vec![0; k as usize];
        for v in sum.iter() {
            count[((*v % k + k) % k) as usize] += 1;
        }
        let mut result: i32 = 0;
        for v in count.iter() {
            result += *v * (*v - 1) / 2;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                a.push(n);
            }
        }
    }

    if 0 == k || 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!(
        "Number of subarrays: {}",
        Solution::subarrays_div_by_k(a, k)
    );
}
