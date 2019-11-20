use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut cur: Vec<Vec<i32>> = vec![Vec::new(); 1];
        let mut map: HashMap<i32, usize> = HashMap::new();
        for n in nums {
            let len: usize = cur.len();
            let start: usize = match map.get(&n) {
                Some(x) => *x,
                None => 0,
            };
            map.insert(n, len);
            for i in start..len {
                if !cur[i].is_empty() && cur[i].last().unwrap() > &n {
                    continue;
                }
                let mut v: Vec<i32> = cur[i].clone();
                v.push(n);
                cur.push(v.clone());
                if v.len() >= 2 {
                    result.push(v);
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
        if 0 != index {
            ret = index;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
            nums.push(number);
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<Vec<i32>> = Solution::find_subsequences(nums);
    for v in result {
        for n in v {
            print!("{} ", n);
        }
        print!("\n");
    }
}
