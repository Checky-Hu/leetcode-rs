use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn get_reverse(num: i32) -> i32 {
        let mut num_mut: i32 = num;
        let mut result: i32 = 0;
        while num_mut > 0 {
            result = result * 10 + num_mut % 10;
            num_mut /= 10;
        }
        result
    }

    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let modulo: i32 = 1_000_000_007;
        let mut result: i32 = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for num in nums.iter() {
            let t: i32 = *num - Solution::get_reverse(*num);
            match map.get_mut(&t) {
                Some(x) => {
                    result = (result + *x) % modulo;
                    *x += 1;
                }
                None => {
                    map.insert(t, 1);
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut nums: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(num);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!("Count nice pairs: {}", Solution::count_nice_pairs(nums));
}
