use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_width_ramp(a: Vec<i32>) -> i32 {
        let len: usize = a.len();
        let mut reorder: Vec<usize> = Vec::with_capacity(len);
        for i in 0..len {
            reorder.push(i);
        }
        reorder.sort_by(|i, j| match a[*i].cmp(&a[*j]) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => i.cmp(&j),
            Ordering::Greater => Ordering::Greater,
        });
        let mut result: usize = 0;
        let mut min: usize = len;
        for v in reorder.iter() {
            if *v > min {
                let t: usize = *v - min;
                if t > result {
                    result = t;
                }
            }
            if *v < min {
                min = *v;
            }
        }
        result as i32
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            a.push(n);
        }
    }

    if ret <= 1 {
        println!("Require at least two parameters.");
        return;
    }

    println!("Max width ramp: {}", Solution::max_width_ramp(a));
}
