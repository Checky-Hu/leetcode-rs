use std::env;

struct Solution {}

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut temp: Vec<u8> = s.into_bytes();
        let mut result: i32 = 0;
        loop {
            let len: usize = temp.len();
            if temp[len - 1] == b'1' {
                if len == 1 {
                    break;
                }
                let mut i: usize = len - 1;
                loop {
                    if temp[i] == b'1' {
                        temp[i] -= 1;
                        if i == 0 {
                            temp.insert(0, b'1');
                            break;
                        } else {
                            i -= 1;
                        }
                    } else {
                        temp[i] += 1;
                        break;
                    }
                }
                result += 1;
            } else {
                temp.pop();
                result += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!("Number of steps: {}", Solution::num_steps(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
