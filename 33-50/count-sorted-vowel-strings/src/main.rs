use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    fn count_vowel_strings_loop(status: &mut Vec<Vec<i32>>, n: usize, index: usize) -> i32 {
        if status[n][index] == 0 {
            if n == 0 {
                status[n][index] = 5 - index as i32;
            } else {
                for i in index..5 {
                    status[n][index] += Solution::count_vowel_strings_loop(status, n - 1, i);
                }
            }
        }
        status[n][index]
    }

    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut status: Vec<Vec<i32>> = vec![vec![0; 5]; 50];
        Solution::count_vowel_strings_loop(&mut status, n as usize - 1, 0)
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let n: i32 = i32::from_str(&arg).expect("Error parse.");
            println!("Sorted vowel strings: {}", Solution::count_vowel_strings(n));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
