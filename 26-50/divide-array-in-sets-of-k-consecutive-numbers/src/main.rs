use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() % k as usize != 0 {
            return false;
        }
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut vec: Vec<i32> = Vec::new();
        for num in nums {
            match map.get_mut(&num) {
                Some(x) => *x += 1,
                None => {
                    map.insert(num, 1);
                    vec.push(num);
                }
            }
        }
        vec.sort();
        while !vec.is_empty() {
            let min: i32 = vec.remove(0);
            let cnt: i32 = map.remove(&min).unwrap();
            if cnt == 0 {
                continue;
            }
            for v in (min + 1)..(min + k) {
                match map.get_mut(&v) {
                    Some(x) => {
                        if *x >= cnt {
                            *x -= cnt;
                        } else {
                            return false;
                        }
                    }
                    None => return false,
                }
            }
        }
        true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
    let mut k: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(num);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
    }

    println!(
        "Possible to divide: {}",
        Solution::is_possible_divide(nums, k)
    );
}
