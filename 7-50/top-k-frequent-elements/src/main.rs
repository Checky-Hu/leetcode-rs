use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for n in &nums {
            match map.get_mut(n) {
                Some(x) => *x += 1,
                None => {
                    map.insert(*n, 1);
                },
            }
        }
        let len: usize = nums.len();
        let mut bucket: Vec<Vec<i32>> = vec![Vec::new(); len + 1];
        for (k, v) in map.iter() {
            bucket[*v as usize].push(*k);
        }
        let mut i: usize = len;
        let mut result: Vec<i32> = Vec::new();
        let mut enough: bool = false;
        loop {
            for j in 0..bucket[i].len() {
                result.push(bucket[i][j]);
                if result.len() == k as usize {
                    enough = true;
                    break;
                }
            }
            if enough {
                break;
            } else {
                i -= 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut k: i32 = 0;
    let mut nums: Vec<i32> = Vec::new();
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
        println!("Require at least two parameters.");
        return
    }

    let result: Vec<i32> = Solution::top_k_frequent(nums, k);
    for r in result {
        print!("{} ", r);
    }
    print!("\n");
}
