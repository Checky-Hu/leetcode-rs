use std::env;

struct Solution {
}

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let len: usize = s.len();
        let mut min: i32 = 0;
        let mut max: i32 = len as i32;
        let mut result: Vec<i32> = Vec::new();
        for c in s.chars() {
            let tmp: i32;
            if c == 'I' {
                tmp = min;
                min += 1;
            } else {
                tmp = max;
                max -= 1;
            }
            result.push(tmp);
        }
        result.push(max);
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            let result: Vec<i32> = Solution::di_string_match(s);
            for n in result {
                print!("{} ", n);
            }
            print!("\n");
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}

