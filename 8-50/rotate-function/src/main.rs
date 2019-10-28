use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn max_rotate_function(a: Vec<i32>) -> i32 {
        let mut sum: i32 = 0;
        let mut result: i32 = 0;
        let len: usize = a.len();
        for i in 0..len {
            sum += a[i];
            result += i as i32 * a[i];
        }
        let mut tmp: i32 = result;
        for i in 1..len {
            tmp = tmp - sum + len as i32 * a[i - 1];
            if tmp > result {
                result = tmp;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                a.push(num);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return
    }

    println!("Max value: {}", Solution::max_rotate_function(a));
}
