use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut len: usize = n as usize;
        let mut vec: Vec<i32> = Vec::with_capacity(len);
        for i in 1..=n {
            vec.push(i);
        }
        let mut index: usize = 0;
        while len > 1 {
            let pos: usize = (index + k as usize - 1) % len;
            vec.remove(pos);
            len -= 1;
            index = pos % len;
        }
        vec[0]
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut n: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let k: i32 = i32::from_str(&arg).expect("Error parse.");
                println!("The winner: {}", Solution::find_the_winner(n, k));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }
}
