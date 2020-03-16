use std::env;

struct Solution {}

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        // 0 => north, 1 => east, 2 => south, 3 => west.
        let mut d: usize = 0;
        let moves: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        for c in instructions.chars() {
            match c {
                'G' => {
                    x += moves[d].0;
                    y += moves[d].1;
                }
                'L' => {
                    if d > 0 {
                        d -= 1;
                    } else {
                        d = 3;
                    }
                }
                _ => {
                    if d < 3 {
                        d += 1;
                    } else {
                        d = 0;
                    }
                }
            }
        }
        d != 0 || (x == 0 && y == 0)
    }
}

fn main() {
    let mut ret: i32 = 0;
    for (index, arg) in env::args().enumerate() {
        if 1 == index {
            ret += 1;
            let instructions: String = arg;
            println!(
                "Is robot bounded: {}",
                Solution::is_robot_bounded(instructions)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
