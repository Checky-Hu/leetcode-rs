use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut result: i32 = 0;
        let len: usize = arr.len();
        for i in 0..(len - 2) {
            for j in (i + 1)..(len - 1) {
                for k in (j + 1)..len {
                    if (arr[i] - arr[j]).abs() <= a
                        && (arr[j] - arr[k]).abs() <= b
                        && (arr[i] - arr[k]).abs() <= c
                    {
                        result += 1;
                    }
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut arr: Vec<i32> = Vec::new();
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut c: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => a = i32::from_str(&arg).expect("Error parse."),
            2 => b = i32::from_str(&arg).expect("Error parse."),
            3 => c = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 3 parameters.");
        return;
    }

    println!(
        "Good triplets: {}",
        Solution::count_good_triplets(arr, a, b, c)
    );
}
