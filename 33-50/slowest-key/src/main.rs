use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut result: (i32, char) = (0, '\0');
        for (i, c) in keys_pressed.chars().enumerate() {
            let t: i32 = if i == 0 {
                release_times[0]
            } else {
                release_times[i] - release_times[i - 1]
            };
            let update: bool = match t.cmp(&result.0) {
                Ordering::Less => false,
                Ordering::Equal => c > result.1,
                Ordering::Greater => true,
            };
            if update {
                result.0 = t;
                result.1 = c;
            }
        }
        result.1
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut keys_pressed: String = String::new();
    let mut release_times: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => keys_pressed = arg,
            _ => {
                ret += 1;
                let release_time: i32 = i32::from_str(&arg).expect("Error parse.");
                release_times.push(release_time);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!(
        "Slowest key: {}",
        Solution::slowest_key(release_times, keys_pressed)
    );
}
