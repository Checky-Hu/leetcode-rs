use std::env;

struct Solution {
}

impl Solution {
    pub fn decode_string(s: String) -> String {
        let bytes: &[u8] = s.as_bytes();
        let len: usize = bytes.len();
        let mut tmp_n: i32 = 0;
        let mut v_n: Vec<i32> = Vec::new();
        let mut tmp_s: String = String::new();
        let mut v_s: Vec<String> = Vec::new();
        let mut i: usize = 0;
        while i < len {
            if bytes[i] == 91 {
                v_n.push(tmp_n);
                tmp_n = 0;
            } else if bytes[i] == 93 {
                let mut s: String = v_s.pop().unwrap();
                let n: i32 = v_n.pop().unwrap();
                for _j in 0..n {
                    s.push_str(&tmp_s);
                }
                tmp_s = s;
            } else if 48 <= bytes[i] && bytes[i] <= 57 {
                if tmp_n == 0 {
                    v_s.push(tmp_s);
                    tmp_s = String::new();
                    tmp_n = bytes[i] as i32 - 48;
                } else {
                    tmp_n = tmp_n * 10 + bytes[i] as i32 - 48;
                }
            } else {
                tmp_s.push(bytes[i] as char);
            }
            i += 1;
        }
        tmp_s
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret = index;
            let s: String = arg;
            println!("Decode string: {}", Solution::decode_string(s));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
