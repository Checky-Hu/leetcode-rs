use std::env;

struct Solution {}

impl Solution {
    pub fn maximum_time(time: String) -> String {
        let mut prefix: char = '0';
        let bytes: &[u8] = time.as_bytes();
        let mut result: String = String::new();
        for i in 0..5 {
            let tmp = if bytes[i] == b'?' {
                match i {
                    0 => {
                        if bytes[1] == b'?' || bytes[1] < b'4' {
                            '2'
                        } else {
                            '1'
                        }
                    }
                    1 => {
                        if prefix == '2' {
                            '3'
                        } else {
                            '9'
                        }
                    }
                    3 => '5',
                    4 => '9',
                    _ => '0',
                }
            } else {
                bytes[i] as char
            };
            result.push(tmp);
            prefix = tmp;
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let time: String = arg;
                println!(
                    "Latest time by replacing hidden digits: {}",
                    Solution::maximum_time(time)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
