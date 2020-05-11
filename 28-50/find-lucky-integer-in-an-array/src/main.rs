use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut counts: Vec<i32> = vec![0; 500];
        for n in arr.iter() {
            counts[*n as usize - 1] += 1;
        }
        let mut result: i32 = -1;
        for (i, n) in counts.iter().enumerate() {
            if i as i32 + 1 == *n {
                result = *n;
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

    println!("Lucky number: {}", Solution::find_lucky(arr));
}
