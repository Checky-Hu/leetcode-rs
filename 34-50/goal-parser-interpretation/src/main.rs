use std::env;

struct Solution {}

impl Solution {
    pub fn interpret(command: String) -> String {
        let mut result: String = String::new();
        let mut find_a: bool = false;
        for c in command.chars() {
            match c {
                'G' => result.push('G'),
                'a' => {
                    find_a = true;
                    result.push('a');
                }
                'l' => result.push('l'),
                ')' => {
                    if find_a {
                        find_a = false;
                    } else {
                        result.push('o');
                    }
                }
                _ => (),
            }
        }
        result
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let command: String = arg;
                println!("After interpret: {}", Solution::interpret(command));
                break;
            }
        }
    }

    if 0 == ret {
        println!("Require at least 1 parameter.");
    }
}
