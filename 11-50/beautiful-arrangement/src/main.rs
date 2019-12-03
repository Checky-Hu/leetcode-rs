use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn count_arrangement_loop(n: i32, v: &mut Vec<i32>) -> i32 {
        if n <= 0 {
            1
        } else {
            let mut result: i32 = 0;
            for i in 0..n {
                if n % v[i as usize] == 0 || v[i as usize] % n == 0 {
                    let t: i32 = v[n as usize - 1];
                    v[n as usize - 1] = v[i as usize];
                    v[i as usize] = t;
                    result += Solution::count_arrangement_loop(n - 1, v);
                    v[i as usize] = v[n as usize - 1];
                    v[n as usize - 1] = t;
                }
            }
            result
        }
    }

    pub fn count_arrangement(n: i32) -> i32 {
        let mut v: Vec<i32> = Vec::with_capacity(n as usize);
        for i in 0..n {
            v.push(i + 1);
        }
        Solution::count_arrangement_loop(n, &mut v)
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Beautiful arrangements: {}", Solution::count_arrangement(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
