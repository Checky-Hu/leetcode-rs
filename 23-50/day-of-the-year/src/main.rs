use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let dates: Vec<i32> = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let strs: Vec<&str> = date.split('-').collect();
        let y: i32 = i32::from_str(strs[0]).expect("Error parse.");
        let m: i32 = i32::from_str(strs[1]).expect("Error parse.");
        let d: i32 = i32::from_str(strs[2]).expect("Error parse.");
        let mut result: i32 = 0;
        let mut i: usize = 0;
        while i < m as usize - 1 {
            result += dates[i];
            if i == 1 && (y % 400 == 0 || (y % 4 == 0 && y % 100 != 0)) {
                result += 1;
            }
            i += 1;
        }
        result + d
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let date: String = arg;
            println!("Day: {}", Solution::day_of_year(date));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}

