use std::env;

struct Solution {}

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let vowels: Vec<u8> = vec![b'a', b'e', b'i', b'o', b'u'];
        let bytes: &[u8] = s.as_bytes();
        let len: usize = bytes.len();
        let mut positions: Vec<usize> = vec![len; 32];
        let mut mask: usize = 0;
        let mut result: usize = 0;
        for (i, src) in bytes.iter().enumerate() {
            for (j, dst) in vowels.iter().enumerate() {
                if *src == *dst {
                    mask ^= 1 << j;
                }
            }
            let t: usize = if mask == 0 {
                i + 1
            } else if i >= positions[mask] {
                i - positions[mask]
            } else {
                0
            };
            if t > result {
                result = t;
            }
            if i < positions[mask] {
                positions[mask] = i;
            }
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
                "Longest substring: {}",
                Solution::find_the_longest_substring(s)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
