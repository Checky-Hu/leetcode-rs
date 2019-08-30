use std::env;

struct Solution {
}

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut result: String = s;
        loop {
            let mut no_duplicates: bool = true;
            unsafe {
                let cur_v: &mut Vec<u8> = result.as_mut_vec();
                let mut i: usize = 0;
                while i < cur_v.len() {
                    if i < cur_v.len() - 1 && cur_v[i + 1] == cur_v[i] {
                        no_duplicates = false;
                        cur_v.remove(i);
                        cur_v.remove(i);
                    } else {
                        i += 1;
                    }
                }
            }
            if no_duplicates {
                break;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            1 => {
	            ret = 1;
                let s: String = arg;
                println!("String: {}", Solution::remove_duplicates(s));
                break;
            },
            _ => (),
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}

