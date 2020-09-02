use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn is_valid(tmp: i32, distance: &[i32], len: usize, target: i32) -> bool {
        let mut index: usize = 0;
        let mut count: i32 = 1;
        for i in 0..len {
            if distance[i] - distance[index] >= tmp {
                index = i;
                count += 1;
                if count >= target {
                    return true;
                }
            }
        }
        false
    }

    pub fn max_distance(position: Vec<i32>, m: i32) -> i32 {
        let mut distance: Vec<i32> = position;
        distance.sort_unstable();
        let len: usize = distance.len();
        let mut max: i32 = distance[len - 1] - distance[0];
        let mut min: i32 = distance[1] - distance[0];
        for i in 2..len {
            let t: i32 = distance[i] - distance[i - 1];
            if t < min {
                min = t;
            }
        }
        while min + 1 < max {
            let mid: i32 = min + (max - min) / 2;
            if Solution::is_valid(mid, &distance, len, m) {
                min = mid;
            } else {
                max = mid - 1;
            }
        }
        if Solution::is_valid(max, &distance, len, m) {
            max
        } else {
            min
        }
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut position: Vec<i32> = Vec::new();
    let mut m: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => m = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                position.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!("Maximum distance: {}", Solution::max_distance(position, m));
}
