use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        let len: i32 = nums.len() as i32;
        let mut flags: Vec<bool> = vec![false; len as usize];
        for i in 0..len {
            if flags[i as usize] {
                continue;
            }
            flags[i as usize] = true;
            let mut map: HashMap<i32, i32> = HashMap::new();
            let mut cur: i32 = i;
            loop {
                let next: i32 = ((cur + nums[cur as usize]) % len + len) % len;
                if next == cur || nums[cur as usize] * nums[next as usize] < 0 {
                    break;
                }
                if map.contains_key(&next) {
                    return true
                }
                map.insert(cur, next);
                cur = next;
                flags[next as usize] = true;
            }
        }
        false
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
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                nums.push(n);
            },
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }

    println!("There is circular: {}", Solution::circular_array_loop(nums));
}
