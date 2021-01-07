use std::env;

struct Solution {}

impl Solution {
    pub fn maximum_binary_string(binary: String) -> String {
        let len: usize = binary.len();
        let mut prefix_0: usize = len;
        for (i, v) in binary.chars().enumerate() {
            if v == '0' {
                if prefix_0 == len {
                    prefix_0 = i;
                } else {
                    prefix_0 += 1;
                }
            }
        }
        let mut bytes: Vec<u8> = vec![49; len];
        if prefix_0 < len {
            bytes[prefix_0] = 48;
        }
        String::from_utf8(bytes).unwrap()
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let binary: String = arg;
                println!(
                    "Maximum binary string: {}",
                    Solution::maximum_binary_string(binary)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
