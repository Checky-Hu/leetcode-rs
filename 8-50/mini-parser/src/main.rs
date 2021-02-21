use std::env;

#[derive(Debug, PartialEq, Eq)]
enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct Solution {}

impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        let mut has_number: bool = false;
        let mut meet_number: bool = false;
        let mut is_positive: bool = true;
        let mut num: i32 = 0;
        let mut val: Vec<NestedInteger> = Vec::new();
        let mut vec: Vec<Vec<NestedInteger>> = Vec::new();
        for c in s.chars() {
            match c {
                '[' => {
                    if has_number {
                        vec.push(val);
                        val = Vec::new();
                    } else {
                        has_number = true;
                    }
                }
                ']' => {
                    if meet_number {
                        meet_number = false;
                        val.push(NestedInteger::Int(if is_positive { num } else { 0 - num }));
                        is_positive = true;
                        num = 0;
                    }
                    if let Some(mut x) = vec.pop() {
                        x.push(NestedInteger::List(val));
                        val = x;
                    }
                }
                ',' => {
                    if meet_number {
                        meet_number = false;
                        val.push(NestedInteger::Int(if is_positive { num } else { 0 - num }));
                        is_positive = true;
                        num = 0;
                    }
                }
                '-' => {
                    is_positive = false;
                }
                _ => {
                    has_number = true;
                    meet_number = true;
                    num = num * 10 + c as i32 - 48;
                }
            }
        }
        if meet_number {
            NestedInteger::Int(if is_positive { num } else { 0 - num })
        } else {
            NestedInteger::List(val)
        }
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let s: String = arg;
            Solution::deserialize(s);
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
