use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let len: usize = arr.len();
        for i in 0..(len - 1) {
            let mut t: i32 = arr[i];
            for j in (i + 1)..len {
                t ^= arr[j];
                if t == 0 {
                    result += (j - i) as i32;
                }
            }
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
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Count triplets: {}", Solution::count_triplets(arr));
}
