use std::env;

struct Solution {}

impl Solution {
    fn get_index(u: u8) -> Option<usize> {
        match u {
            b'Q' => Some(0),
            b'W' => Some(1),
            b'E' => Some(2),
            b'R' => Some(3),
            _ => None,
        }
    }

    pub fn balanced_string(s: String) -> i32 {
        let bytes: &[u8] = s.as_bytes();
        let len: usize = bytes.len();
        let target: usize = len / 4;
        // [Q, W, E, R]
        let mut counts: Vec<usize> = vec![0; 4];
        for u in bytes.iter() {
            if let Some(x) = Solution::get_index(*u) {
                counts[x] += 1;
            }
        }
        let mut result: i32 = len as i32;
        let mut slow: usize = 0;
        let mut fast: usize = 0;
        while fast < len {
            if let Some(x) = Solution::get_index(bytes[fast]) {
                counts[x] -= 1;
                fast += 1;
                while slow < len
                    && counts[0] <= target
                    && counts[1] <= target
                    && counts[2] <= target
                    && counts[3] <= target
                {
                    let t: i32 = (fast - slow) as i32;
                    if t < result {
                        result = t;
                    }
                    if let Some(y) = Solution::get_index(bytes[slow]) {
                        counts[y] += 1;
                    }
                    slow += 1;
                }
            } else {
                fast += 1;
            }
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let s: String = arg;
                println!("Balanced string: {}", Solution::balanced_string(s));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
