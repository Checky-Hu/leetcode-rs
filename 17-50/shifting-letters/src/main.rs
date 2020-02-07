use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        let len: usize = shifts.len();
        let mut tmp: Vec<i32> = vec![0; len + 1];
        let mut i: usize = len;
        loop {
            tmp[i - 1] = tmp[i] % 26 + shifts[i - 1] % 26;
            i -= 1;
            if i == 0 {
                break;
            }
        }
        let mut result: String = String::new();
        for (i, u) in s.as_bytes().iter().enumerate() {
            let t: u8 = (*u - 97 + tmp[i] as u8) % 26 + 97;
            result.push(t as char);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut s: String = String::new();
    let mut shifts: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => s = arg,
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                shifts.push(t);
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
        return;
    }

    println!(
        "Shifting letters: {}",
        Solution::shifting_letters(s, shifts)
    );
}
