use std::env;

struct Solution {}

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let bytes: &[u8] = coordinates.as_bytes();
        if (bytes[0] - 97) & 1 == 0 {
            bytes[1] & 1 == 0
        } else {
            bytes[1] & 1 == 1
        }
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let coordinates: String = arg;
                println!(
                    "Square is white: {}",
                    Solution::square_is_white(coordinates)
                );
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
