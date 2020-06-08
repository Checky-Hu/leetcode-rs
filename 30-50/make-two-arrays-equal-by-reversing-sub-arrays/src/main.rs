use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        let mut count_t: Vec<i32> = vec![0; 1000];
        for v in target.iter() {
            count_t[*v as usize - 1] += 1;
        }
        let mut count_a: Vec<i32> = vec![0; 1000];
        for v in arr.iter() {
            count_a[*v as usize - 1] += 1;
        }
        for i in 0..1000 {
            if count_t[i] != count_a[i] {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut target: Vec<i32> = Vec::new();
    let mut arr: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                if target.len() == n as usize {
                    arr.push(t);
                } else {
                    target.push(t);
                }
            }
        }
    }

    if 0 == n || 2 * n != ret {
        println!("Require at least (1 + 2 * arg1) parameters.");
        return;
    }

    println!("Can be equal: {}", Solution::can_be_equal(target, arr));
}
