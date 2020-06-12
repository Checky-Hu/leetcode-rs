use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut num: i32 = low;
        let mut min: usize = 0;
        while num > 0 {
            min += 1;
            num /= 10;
        }
        let mut num: i32 = high;
        let mut max: usize = 0;
        while num > 0 {
            max += 1;
            num /= 10;
        }
        let mut result: Vec<i32> = Vec::new();
        for i in min..=max {
            for j in 1..=9 {
                let mut t: i32 = j;
                let mut tmp: i32 = 0;
                let mut is_overflow: bool = false;
                for _k in 0..i {
                    if t > 9 {
                        is_overflow = true;
                        break;
                    } else {
                        tmp = tmp * 10 + t;
                        t += 1;
                    }
                }
                if is_overflow || tmp > high {
                    break;
                } else if low > tmp {
                    continue;
                } else {
                    result.push(tmp);
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut low: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => low = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let high: i32 = i32::from_str(&arg).expect("Error parse.");
                let result: Vec<i32> = Solution::sequential_digits(low, high);
                for r in result.iter() {
                    print!("{} ", *r);
                }
                println!();
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least two parameters.");
    }
}
