use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let len: usize = arr.len();
        let mut visits: Vec<bool> = vec![false; len];
        let mut queue: Vec<usize> = vec![start as usize];
        while !queue.is_empty() {
            let mut next: Vec<usize> = Vec::new();
            for q in queue {
                if arr[q] == 0 {
                    return true;
                }
                visits[q] = true;
                let mut t: usize = q + arr[q] as usize;
                if t < len && !visits[t] {
                    next.push(t);
                }
                if q >= arr[q] as usize {
                    t = q - arr[q] as usize;
                    if !visits[t] {
                        next.push(t);
                    }
                }
            }
            queue = next;
        }
        false
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut arr: Vec<i32> = Vec::new();
    let mut start: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => start = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(num);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }

    println!("Can reach 0 value: {}", Solution::can_reach(arr, start));
}
