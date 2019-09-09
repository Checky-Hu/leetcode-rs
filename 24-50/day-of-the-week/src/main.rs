use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let result: Vec<&str> = vec!["Sunday", "Monday", "Tuesday",
            "Wednesday", "Thursday", "Friday", "Saturday"];
        let (m, c, y) = if month < 3 {
            (12 + month, (year - 1) / 100, (year - 1) % 100)
        } else {
            (month, year / 100, year % 100)
        };
        let mut tmp: i32 = y + y / 4 + c / 4 - 2 * c + 26 * (m + 1) / 10 + day - 1;
        if tmp < 0 {
            tmp = tmp % 7 + 7;
        }
        result[ tmp as usize % 7].to_string()
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut day: i32 = 0;
    let mut month: i32 = 0;
    let mut year: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            day = i32::from_str(&arg).expect("Error parse.");
        } else if 2 == index {
            ret += 1;
            month = i32::from_str(&arg).expect("Error parse.");
        } else if 3 == index {
            ret += 1;
            year = i32::from_str(&arg).expect("Error parse.");
            break;
        } else {
        }
    }

    if 3 != ret {
        println!("Require at least three parameters.");
        return;
    }

    println!("Day: {}", Solution::day_of_the_week(day, month, year));
}
