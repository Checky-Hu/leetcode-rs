use std::env;

struct Solution {}

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut r_queue: Vec<usize> = Vec::new();
        let mut d_queue: Vec<usize> = Vec::new();
        for (i, c) in senate.chars().enumerate() {
            if c == 'R' {
                r_queue.push(i);
            } else {
                d_queue.push(i);
            }
        }
        let len: usize = senate.len();
        while !r_queue.is_empty() && !d_queue.is_empty() {
            let r: usize = r_queue.remove(0);
            let d: usize = d_queue.remove(0);
            if r < d {
                r_queue.push(r + len);
            } else {
                d_queue.push(d + len);
            }
        }
        if r_queue.is_empty() {
            "Dire".to_string()
        } else {
            "Radiant".to_string()
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let senate: String = arg;
            println!("Victory party: {}", Solution::predict_party_victory(senate));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
