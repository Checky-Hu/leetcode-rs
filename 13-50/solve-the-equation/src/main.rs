use std::env;

struct Solution {}

impl Solution {
    pub fn solve_equation(equation: String) -> String {
        let mut a: i32 = 0;
        let mut b: i32 = 0;
        let mut cur: i32 = -1;
        let mut is_add: bool = true;
        let mut is_equal: bool = false;
        for c in equation.chars() {
            match c {
                '+' | '-' => {
                    if cur != -1 {
                        if is_add {
                            b += cur;
                        } else {
                            b -= cur;
                        }
                        cur = -1;
                    }
                    if c == '+' {
                        is_add = !is_equal;
                    } else {
                        is_add = is_equal;
                    }
                }
                'x' => {
                    if cur == -1 {
                        cur = 1;
                    }
                    if is_add {
                        a += cur;
                    } else {
                        a -= cur;
                    }
                    cur = -1;
                }
                '=' => {
                    if cur != -1 {
                        if is_add {
                            b += cur;
                        } else {
                            b -= cur;
                        }
                        cur = -1;
                    }
                    is_equal = true;
                    is_add = false;
                }
                _ => cur = (c as u8 - 48) as i32 + if cur == -1 { 0 } else { cur * 10 },
            }
        }
        if cur != -1 {
            if is_add {
                b += cur;
            } else {
                b -= cur;
            }
        }
        if a == 0 {
            if b == 0 {
                "Infinite solutions".to_string()
            } else {
                "No solution".to_string()
            }
        } else {
            "x=".to_string() + &(0 - b / a).to_string()
        }
    }
}

fn main() {
    let mut ret: usize = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let equation: String = arg;
            println!("Result: {}", Solution::solve_equation(equation));
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
