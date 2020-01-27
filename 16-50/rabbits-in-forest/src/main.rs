use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for answer in &answers {
            match map.get_mut(answer) {
                Some(x) => *x += 1,
                None => {
                    map.insert(*answer, 1);
                }
            }
        }
        let mut result: i32 = 0;
        for (k, v) in map.iter() {
            let mut tmp: i32 = *v;
            while tmp > 0 {
                result += *k + 1;
                tmp -= *k + 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut answers: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                answers.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Num of rabbits: {}", Solution::num_rabbits(answers));
}
