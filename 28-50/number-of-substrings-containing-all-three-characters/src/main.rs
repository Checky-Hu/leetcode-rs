use std::env;

struct Solution {}

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = 0;
        let mut count: Vec<i32> = vec![0; 3];
        let bytes: &[u8] = s.as_bytes();
        let len: usize = bytes.len();
        let mut result: usize = 0;
        while right < len {
            count[bytes[right] as usize - 97] += 1;
            while count[0] > 0 && count[1] > 0 && count[2] > 0 {
                result += len - right;
                count[bytes[left] as usize - 97] -= 1;
                left += 1;
            }
            right += 1;
        }
        result as i32
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!(
                "Number of substrings: {}",
                Solution::number_of_substrings(s)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
