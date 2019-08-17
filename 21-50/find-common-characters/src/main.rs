use std::env;

struct Solution {
}

impl Solution {
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        let mut min: Vec<i32> = vec![0; 26];
        let mut is_first: bool = true;
        for s in a {
            let mut tmp: Vec<i32> = vec![0; 26];
            for c in s.chars() {
                tmp[(c as u8 - 97) as usize] += 1;
            }
            if is_first {
                let mut i: usize = 0;
                while i < 26 {
                    min[i] = tmp[i];
                    i += 1;
                }
                is_first = false;
            } else {
                let mut i: usize = 0;
                while i < 26 {
                    if tmp[i] < min[i] {
                        min[i] = tmp[i];
                    }
                    i += 1;
                }
            }
        }

        let mut result: Vec<String> = Vec::new();
        let mut tmp_c: u8 = 97;
        for n in min {
            let mut i: i32 = 0;
            while i < n {
                result.push((tmp_c as char).to_string());
                i += 1;
            }
            tmp_c += 1;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut a: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let s: String = arg;
                a.push(s);
            },
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<String> = Solution::common_chars(a);
    for s in result {
        println!("{}", s);
    }
}

