use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn find_latest_step(arr: Vec<i32>, m: i32) -> i32 {
        let len: i32 = arr.len() as i32;
        if len < m {
            return -1;
        } else if len == m {
            return len;
        }
        let mut state: Vec<(i32, i32)> = vec![(1, len)];
        let mut index: usize = len as usize - 1;
        loop {
            let mut j: usize = len as usize;
            for (i, v) in state.iter().enumerate() {
                if v.0 <= arr[index] && arr[index] <= v.1 {
                    j = i;
                    break;
                }
            }
            if state[j].0 == state[j].1 {
                state.remove(j);
            } else if state[j].0 == arr[index] {
                if state[j].1 - state[j].0 == m {
                    return index as i32;
                }
                state[j].0 = arr[index] + 1;
            } else if state[j].1 == arr[index] {
                if state[j].1 - state[j].0 == m {
                    return index as i32;
                }
                state[j].1 = arr[index] - 1;
            } else {
                if arr[index] - state[j].0 == m || state[j].1 - arr[index] == m {
                    return index as i32;
                }
                let t: i32 = state[j].1;
                state[j].1 = arr[index] - 1;
                state.insert(j + 1, (arr[index] + 1, t));
            }
            if index == 0 {
                break;
            } else {
                index -= 1;
            }
        }
        -1
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut arr: Vec<i32> = Vec::new();
    let mut m: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => m = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(n);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 2 parameters.");
        return;
    }

    println!(
        "Latest group of size {}: {}",
        m,
        Solution::find_latest_step(arr, m)
    );
}
