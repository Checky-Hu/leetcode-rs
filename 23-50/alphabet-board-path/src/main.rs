use std::env;

struct Solution {}

impl Solution {
    pub fn alphabet_board_path(target: String) -> String {
        let mut position: (u8, u8) = (0, 0);
        let mut result: String = String::new();
        for u in target.as_bytes().iter() {
            let x: u8 = (*u - 97) % 5;
            let y: u8 = (*u - 97) / 5;
            let (h_t, h_c) = if x >= position.0 {
                (x - position.0, 'R')
            } else {
                (position.0 - x, 'L')
            };
            let (v_t, v_c) = if y >= position.1 {
                (y - position.1, 'D')
            } else {
                (position.1 - y, 'U')
            };
            let (f_t, f_c, s_t, s_c) = if *u == 122 {
                (h_t, h_c, v_t, v_c)
            } else {
                (v_t, v_c, h_t, h_c)
            };
            for _i in 0..f_t {
                result.push(f_c);
            }
            for _i in 0..s_t {
                result.push(s_c);
            }
            result.push('!');
            position.0 = x;
            position.1 = y;
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let target: String = arg;
            println!(
                "Alphabet board path: {}",
                Solution::alphabet_board_path(target)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
