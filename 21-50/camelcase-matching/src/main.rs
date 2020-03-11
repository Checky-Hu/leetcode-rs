use std::env;

struct Solution {}

impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let mut upper: Vec<char> = Vec::new();
        let mut lower: Vec<Vec<char>> = Vec::new();
        let mut t: Vec<char> = Vec::new();
        for c in pattern.chars() {
            if c.is_ascii_uppercase() {
                lower.push(t);
                t = Vec::new();
                upper.push(c);
            } else {
                t.push(c);
            }
        }
        lower.push(t);
        let len: usize = upper.len();
        let mut result: Vec<bool> = Vec::new();
        for query in queries.iter() {
            let mut is_match: bool = true;
            let mut upper_i: usize = 0;
            let mut lower_i: usize = 0;
            for c in query.chars() {
                if c.is_ascii_uppercase() {
                    if upper_i < len && c == upper[upper_i] && lower_i == lower[upper_i].len() {
                        lower_i = 0;
                        upper_i += 1;
                    } else {
                        is_match = false;
                        break;
                    }
                } else if lower_i < lower[upper_i].len() && c == lower[upper_i][lower_i] {
                    lower_i += 1;
                }
            }
            result.push(is_match && upper_i == len && lower_i == lower[upper_i].len());
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut pattern: String = String::new();
    let mut queries: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => pattern = arg,
            _ => {
                ret += 1;
                let s: String = arg;
                queries.push(s);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
        return;
    }

    let result: Vec<bool> = Solution::camel_match(queries, pattern);
    for s in result.iter() {
        print!("{} ", *s);
    }
    println!();
}
