use std::env;

struct Solution {}

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut counts: Vec<i32> = vec![0; 5];
        for u in text.as_bytes() {
            match u {
                b'a' => counts[0] += 1,
                b'b' => counts[1] += 1,
                b'l' => counts[2] += 1,
                b'n' => counts[3] += 1,
                b'o' => counts[4] += 1,
                _ => (),
            }
        }
        let mut result: i32 = i32::max_value();
        for (i, v) in counts.iter().enumerate() {
            let t: i32 = if i == 2 || i == 4 { *v / 2 } else { *v };
            if t < result {
                result = t;
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
            println!(
                "Maximum number of balloons: {}",
                Solution::max_number_of_balloons(text)
            );
            break;
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
}
