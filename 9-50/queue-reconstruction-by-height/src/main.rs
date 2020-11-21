use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let len: usize = people.len();
        let mut result: Vec<Vec<i32>> = people;
        result.sort_by(|a, b| b[0].cmp(&a[0]));
        let mut i: usize = 0;
        while i < len {
            let target: usize = result[i][1] as usize;
            if target < i {
                let t: Vec<i32> = result.remove(i);
                result.insert(target, t);
            }
            i += 1;
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut people: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(t);
                if tmp.len() == 2 {
                    people.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 2 > ret {
        println!("Require at least 2 parameters.");
        return;
    }

    let result: Vec<Vec<i32>> = Solution::reconstruct_queue(people);
    for r in result.iter() {
        print!("[{}, {}] ", r[0], r[1]);
    }
    println!();
}
