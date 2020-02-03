use std::env;

struct Solution {}

impl Solution {
    fn helper(bytes: &[u8], s: usize, e: usize) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let tmp: String =
            String::from_utf8(bytes.get(s..=e).unwrap_or(&[0]).to_vec()).unwrap_or_default();
        if s == e || bytes[s] != b'0' {
            result.push(tmp.clone());
        }
        if bytes[e] != b'0' {
            for i in 1..=(e - s) {
                if i > 1 && bytes[s] == b'0' {
                    break;
                }
                let mut cur: String = tmp.clone();
                cur.insert(i, '.');
                result.push(cur);
            }
        }
        result
    }

    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let bytes: &[u8] = s.as_bytes();
        let len: usize = bytes.len();
        let mut result: Vec<String> = Vec::new();
        for i in 2..(len - 1) {
            let n1: Vec<String> = Solution::helper(bytes, 1, i - 1);
            let n2: Vec<String> = Solution::helper(bytes, i, len - 2);
            for s1 in &n1 {
                for s2 in &n2 {
                    let mut tmp: String = String::new();
                    tmp.push('(');
                    tmp.push_str(s1);
                    tmp.push_str(", ");
                    tmp.push_str(s2);
                    tmp.push(')');
                    result.push(tmp);
                }
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
            let result: Vec<String> = Solution::ambiguous_coordinates(s);
            for r in result {
                println!("{}", r);
            }
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
