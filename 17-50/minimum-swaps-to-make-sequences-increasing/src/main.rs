use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn min_swap(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let mut swap: i32 = 1;
        let mut no_swap: i32 = 0;
        for i in 1..a.len() {
            let mut cur_swap: i32 = i32::max_value();
            let mut cur_no_swap: i32 = i32::max_value();
            if a[i] > a[i - 1] && b[i] > b[i - 1] {
                cur_swap = if cur_swap < swap + 1 {
                    cur_swap
                } else {
                    swap + 1
                };
                cur_no_swap = if cur_no_swap < no_swap {
                    cur_no_swap
                } else {
                    no_swap
                };
            }
            if a[i] > b[i - 1] && b[i] > a[i - 1] {
                cur_swap = if cur_swap < no_swap + 1 {
                    cur_swap
                } else {
                    no_swap + 1
                };
                cur_no_swap = if cur_no_swap < swap {
                    cur_no_swap
                } else {
                    swap
                };
            }
            swap = cur_swap;
            no_swap = cur_no_swap;
        }
        if swap < no_swap {
            swap
        } else {
            no_swap
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut a: Vec<i32> = Vec::new();
    let mut b: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                if a.len() as i32 == n {
                    b.push(t);
                } else {
                    a.push(t);
                }
            }
        }
    }

    if 0 == n || 2 * n != ret {
        println!("Require at least (1 + 2 * arg1) parameters.");
        return;
    }

    println!("Min swaps: {}", Solution::min_swap(a, b));
}
