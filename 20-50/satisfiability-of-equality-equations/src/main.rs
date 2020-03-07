use std::env;

struct Solution {}

impl Solution {
    pub fn equation_possible(equations: Vec<String>) -> bool {
        // u8 means: 0 => unknown, 1 => equal, 2 => not equal.
        let mut equals: Vec<Vec<usize>> = vec![Vec::new(); 26];
        let mut no_equals: Vec<Vec<usize>> = vec![Vec::new(); 26];
        for s in equations.iter() {
            let bytes: &[u8] = s.as_bytes();
            if bytes[0] == bytes[3] {
                if bytes[1] == b'=' {
                    continue;
                } else {
                    return false;
                }
            } else if bytes[1] == b'=' {
                equals[(bytes[0] - b'a') as usize].push((bytes[3] - b'a') as usize);
                equals[(bytes[3] - b'a') as usize].push((bytes[0] - b'a') as usize);
            } else {
                no_equals[(bytes[0] - b'a') as usize].push((bytes[3] - b'a') as usize);
            }
        }
        let mut values: Vec<i32> = vec![0; 26];
        let mut value: i32 = 1;
        for i in 0..26 {
            if values[i] != 0 {
                continue;
            }
            let mut queue: Vec<usize> = vec![i];
            values[i] = value;
            while !queue.is_empty() {
                let mut temp: Vec<usize> = Vec::new();
                for pos in queue {
                    for u in equals[pos].iter() {
                        if values[*u] == 0 {
                            temp.push(*u);
                            values[*u] = value;
                        }
                    }
                }
                queue = temp;
            }
            value += 1;
        }
        for (i, vec) in no_equals.iter().enumerate() {
            for pos in vec.iter() {
                if values[i] != 0 && values[i] == values[*pos] {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut equations: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        if 0 != index {
            ret += 1;
            let s: String = arg;
            equations.push(s);
        }
    }

    if ret == 0 {
        println!("Require at least one parameter.");
        return;
    }

    println!(
        "Equation possible: {}",
        Solution::equation_possible(equations)
    );
}
