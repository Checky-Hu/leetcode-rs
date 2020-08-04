use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        if k == 1 {
            return true;
        }
        let mut positives: Vec<i32> = vec![0; k as usize];
        let mut negatives: Vec<i32> = vec![0; k as usize];
        for n in arr.iter() {
            let t: i32 = *n % k;
            if t >= 0 {
                positives[t as usize] += 1;
            } else {
                negatives[(0 - t) as usize] += 1;
            }
        }
        for i in 1..=(k >> 1) {
            if i != k - i {
                if positives[i as usize] - negatives[i as usize]
                    != positives[(k - i) as usize] - negatives[(k - i) as usize]
                {
                    return false;
                }
            } else {
                let t: i32 = positives[i as usize] - negatives[i as usize];
                if t & 1 == 1 || (i << 1) % k != 0 {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut k: i32 = 0;
    let mut arr: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => k = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                arr.push(t);
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
        return;
    }

    println!("Can arrange to {}: {}", k, Solution::can_arrange(arr, k));
}
