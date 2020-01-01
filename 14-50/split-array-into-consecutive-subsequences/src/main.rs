use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn is_possible(nums: Vec<i32>) -> bool {
        let mut count: HashMap<i32, i32> = HashMap::new();
        for n in &nums {
            match count.get_mut(n) {
                Some(a) => *a += 1,
                None => {
                    count.insert(*n, 1);
                }
            }
        }
        let mut need: HashMap<i32, i32> = HashMap::new();
        for n in &nums {
            let n_count: i32 = match count.get(n) {
                Some(x) => *x,
                None => 0,
            };
            if n_count == 0 {
                continue;
            }
            let n_need: i32 = match need.get(n) {
                Some(x) => *x,
                None => 0,
            };
            if n_need > 0 {
                need.insert(*n, n_need - 1);
                match need.get_mut(&(*n + 1)) {
                    Some(x) => *x += 1,
                    None => {
                        need.insert(*n + 1, 1);
                    }
                }
            } else {
                let n1_count: i32 = match count.get(&(*n + 1)) {
                    Some(x) => *x,
                    None => 0,
                };
                let n2_count: i32 = match count.get(&(*n + 2)) {
                    Some(x) => *x,
                    None => 0,
                };
                if n1_count > 0 && n2_count > 0 {
                    count.insert(*n + 1, n1_count - 1);
                    count.insert(*n + 2, n2_count - 1);
                    match need.get_mut(&(*n + 3)) {
                        Some(x) => *x += 1,
                        None => {
                            need.insert(*n + 3, 1);
                        }
                    }
                } else {
                    return false;
                }
            }
            count.insert(*n, n_count - 1);
        }
        true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let number: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(number);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    println!("Possible to split: {}", Solution::is_possible(nums));
}
