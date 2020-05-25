use std::env;

struct Solution {}

impl Solution {
    pub fn entity_parser(text: String) -> String {
        let bytes: &[u8] = text.as_bytes();
        let len: usize = bytes.len();
        let mut result: String = String::with_capacity(len);
        let mut i: usize = 0;
        while i < len {
            if bytes[i] == b'&' {
                if i + 5 < len
                    && bytes[i + 1] == b'q'
                    && bytes[i + 2] == b'u'
                    && bytes[i + 3] == b'o'
                    && bytes[i + 4] == b't'
                    && bytes[i + 5] == b';'
                {
                    result.push('"');
                    i += 6;
                } else if i + 5 < len
                    && bytes[i + 1] == b'a'
                    && bytes[i + 2] == b'p'
                    && bytes[i + 3] == b'o'
                    && bytes[i + 4] == b's'
                    && bytes[i + 5] == b';'
                {
                    result.push('\'');
                    i += 6;
                } else if i + 4 < len
                    && bytes[i + 1] == b'a'
                    && bytes[i + 2] == b'm'
                    && bytes[i + 3] == b'p'
                    && bytes[i + 4] == b';'
                {
                    result.push('&');
                    i += 5;
                } else if i + 3 < len
                    && bytes[i + 1] == b'g'
                    && bytes[i + 2] == b't'
                    && bytes[i + 3] == b';'
                {
                    result.push('>');
                    i += 4;
                } else if i + 3 < len
                    && bytes[i + 1] == b'l'
                    && bytes[i + 2] == b't'
                    && bytes[i + 3] == b';'
                {
                    result.push('<');
                    i += 4;
                } else if i + 6 < len
                    && bytes[i + 1] == b'f'
                    && bytes[i + 2] == b'r'
                    && bytes[i + 3] == b'a'
                    && bytes[i + 4] == b's'
                    && bytes[i + 5] == b'l'
                    && bytes[i + 6] == b';'
                {
                    result.push('/');
                    i += 7;
                } else {
                    result.push('&');
                    i += 1;
                }
            } else {
                result.push(bytes[i] as char);
                i += 1;
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
            let text: String = arg;
            println!("Entity parser: {}", Solution::entity_parser(text));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
