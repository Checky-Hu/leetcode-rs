extern crate quicksort;

use std::env;
use std::str::FromStr;
use quicksort::qsi32;

struct Solution {
}

impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let len1: usize = houses.len();
        let len2: usize = heaters.len();
        if len1 == 0 || len2 == 0 {
            return 0
        }
        let mut tmp_houses: Vec<i32> = houses.clone();
        qsi32::quick_sort(&mut tmp_houses, 0, len1 - 1);
        let mut tmp_heaters: Vec<i32> = heaters.clone();
        qsi32::quick_sort(&mut tmp_heaters, 0, len2 - 1);
        let mut cur_i: usize = 0;
        let mut result: i32 = 0;
        for house in tmp_houses {
            while cur_i + 1 < len2 {
                let t1: i32 = if house >= tmp_heaters[cur_i] {
                    house - tmp_heaters[cur_i]
                } else {
                    tmp_heaters[cur_i] - house
                };
                let t2: i32 = if house >= tmp_heaters[cur_i + 1] {
                    house - tmp_heaters[cur_i + 1]
                } else {
                    tmp_heaters[cur_i + 1] - house
                };
                if t2 <= t1 {
                    cur_i += 1;
                } else {
                    break;
                }
            }
            let tmp_r: i32 = if house >= tmp_heaters[cur_i] {
                house - tmp_heaters[cur_i]
            } else {
                tmp_heaters[cur_i] - house
            };
            if tmp_r > result {
                result = tmp_r;
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut n: i32 = 0;
    let mut houses: Vec<i32> = Vec::new();
    let mut heaters: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            n = i32::from_str(&arg).expect("Error parse.");
        } else if 1 < index {
            ret += 1;
            let number: i32 = i32::from_str(&arg).expect("Error parse.");
            if houses.len() < n as usize {
                houses.push(number);
            } else {
                heaters.push(number);
            }
        }
    }

    if 0 == ret || n as usize > ret {
        println!("Require at least (n + 1) parameters.");
        return;
    }

    println!("Radius: {}", Solution::find_radius(houses, heaters));
}
