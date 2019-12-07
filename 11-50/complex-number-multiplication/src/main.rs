use std::env;

struct Solution {
}

impl Solution {
    fn get_param(s: &String) -> Vec<i32> {
        let mut param: Vec<i32> = vec![0; 2];
        let mut is_pos: Vec<bool> = vec![true; 2];
        let mut find_plus: bool = false;
        for c in s.chars() {
            if find_plus {
                match c {
                    '-' => is_pos[1] = false,
                    'i' => break,
                    _ => param[1] = param[1] * 10 + (c as u8 - 48) as i32,
                }
            } else {
                match c {
                    '-' => is_pos[0] = false,
                    '+' => find_plus = true,
                    _ => param[0] = param[0] * 10 + (c as u8 - 48) as i32,
                }
            }
        }
        for i in 0..2 {
            if !is_pos[i] {
                param[i] *= -1;
            }
        }
        param
    }

    pub fn complex_number_multiply(a: String, b: String) -> String {
        let p1: Vec<i32> = Solution::get_param(&a);
        let p2: Vec<i32> = Solution::get_param(&b);
        let x: i32 = p1[0] * p2[0] - p1[1] * p2[1];
        let y: i32 = p1[0] * p2[1] + p1[1] * p2[0];
        x.to_string() + "+" + &y.to_string() + "i"
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut a: String = String::new();
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            a = arg;
        } else if 2 == index {
            ret += 1;
            let b: String = arg;
            println!("Multiply result: {}", Solution::complex_number_multiply(a, b));
            break;
        }
    }

    if 2 > ret {
        println!("Require at least two parameters.");
    }
}
