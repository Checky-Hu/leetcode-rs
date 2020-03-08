use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn longest_ones(a: Vec<i32>, k: i32) -> i32 {
        let mut rest: i32 = k;
        let mut queue: Vec<usize> = Vec::new();
        let mut result: i32 = 0;
        let mut cur_l: i32 = 0;
        for (i, v) in a.iter().enumerate() {
            if *v == 0 {
                if rest == 0 {
                    if cur_l > result {
                        result = cur_l;
                    }
                    match queue.first() {
                        Some(x) => {
                            cur_l = (i - x) as i32;
                            queue.remove(0);
                            queue.push(i);
                        }
                        None => cur_l = 0,
                    }
                } else {
                    rest -= 1;
                    cur_l += 1;
                    queue.push(i);
                }
            } else {
                cur_l += 1;
            }
        }
        if cur_l > result {
            result = cur_l;
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

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Max consecutive ones: {}", Solution::longest_ones(a, k));
}
