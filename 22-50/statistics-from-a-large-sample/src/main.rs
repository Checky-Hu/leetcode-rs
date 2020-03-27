use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn sample_stats(count: Vec<i32>) -> Vec<f64> {
        let mut meet_min: bool = false;
        let mut pre_num: i32 = 0;
        let mut num: Vec<i32> = vec![0; 256];
        let mut sum: f64 = 0_f64;
        let mut mode: (i32, f64) = (0, 0_f64);
        let mut result: Vec<f64> = vec![0_f64; 5];
        for (i, v) in count.iter().enumerate() {
            if *v != 0 {
                if !meet_min {
                    result[0] = i as f64;
                    meet_min = true;
                }
                result[1] = i as f64;
                sum += i as f64 * *v as f64;
                pre_num += *v;
                num[i] = pre_num;
                if *v > mode.0 {
                    mode.0 = *v;
                    mode.1 = i as f64;
                }
            }
        }
        result[2] = sum / pre_num as f64;
        result[4] = mode.1;
        let is_odd: bool = pre_num & 1 == 1;
        let mut target: i32 = pre_num / 2;
        let mut meet_first: bool = false;
        for (i, v) in num.iter().enumerate() {
            if *v >= target {
                if is_odd {
                    result[3] = i as f64;
                    break;
                } else if meet_first {
                    result[3] = (result[3] + i as f64) / 2_f64;
                    break;
                } else {
                    result[3] = i as f64;
                    meet_first = true;
                    target += 1;
                    if *v >= target {
                        break;
                    }
                }
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut count: Vec<i32> = vec![0; 256];
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                count[ret] = n;
                ret += 1;
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<f64> = Solution::sample_stats(count);
    for n in result.iter() {
        print!("{} ", *n);
    }
    println!();
}
