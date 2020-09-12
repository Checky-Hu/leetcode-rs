use std::env;

struct Solution {}

impl Solution {
    pub fn num_ways(s: String) -> i32 {
        let modulo: i64 = 1_000_000_007;
        let bytes: &[u8] = s.as_bytes();
        let len: usize = bytes.len();
        let mut count: i32 = 0;
        for v in bytes.iter() {
            if *v == b'1' {
                count += 1;
            }
        }
        if count % 3 != 0 {
            return 0;
        }
        if count == 0 {
            let from: i64 = len as i64 - 1;
            return ((from * (from - 1) / 2) % modulo) as i32;
        }
        let target: i32 = count / 3;
        count = 0;
        let mut prefix: i64 = 1;
        for v in bytes.iter() {
            if *v == b'1' {
                if count == target {
                    break;
                } else {
                    count += 1;
                }
            } else if count == target {
                prefix += 1;
            }
        }
        count = 0;
        let mut suffix: i64 = 1;
        let mut index: usize = len - 1;
        loop {
            if bytes[index] == b'1' {
                if count == target {
                    break;
                } else {
                    count += 1;
                }
            } else if count == target {
                suffix += 1;
            }
            index -= 1;
        }
        (prefix * suffix % modulo) as i32
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            println!("Number of ways: {}", Solution::num_ways(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
