use std::env;
use std::str::FromStr;

struct Solution {
}

impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; num as usize + 1];
        for i in 1..=num {
            result[i as usize]= result[(i & (i - 1)) as usize] + 1;
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            1 => {
                ret += 1;
                let num: i32 = i32::from_str(&arg).expect("Error parse.");
                let result: Vec<i32> = Solution::count_bits(num);
                for r in result {
                    print!("{} ", r);
                }
                print!("\n");
                break;
            },
            _ => (),
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
