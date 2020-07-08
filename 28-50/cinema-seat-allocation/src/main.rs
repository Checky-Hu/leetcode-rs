use std::collections::HashMap;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mask_1: i32 = 0x1E;
        let mask_2: i32 = 0x1FE;
        let mask_3: i32 = 0x60;
        let mask_4: i32 = 0x18;
        let mask_5: i32 = 0x180;
        let mut seats: HashMap<i32, i32> = HashMap::new();
        for reserved_seat in reserved_seats.iter() {
            let key: i32 = reserved_seat[0] - 1;
            let mask: i32 = 1 << (reserved_seat[1] - 1);
            match seats.get_mut(&key) {
                Some(x) => *x ^= mask,
                None => {
                    seats.insert(key, 0x3FF ^ mask);
                }
            }
        }
        let mut result: i32 = 0;
        for v in seats.values() {
            if *v & mask_1 == mask_1 {
                result += 1;
                if *v & mask_2 == mask_2 {
                    result += 1;
                }
            } else if (*v & mask_3 == mask_3)
                && ((*v & mask_4 == mask_4) || (*v & mask_5 == mask_5))
            {
                result += 1;
            }
        }
        result + (n - seats.len() as i32) * 2
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut n: i32 = 0;
    let mut reserved_seats: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => n = i32::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let t: i32 = i32::from_str(&arg).expect("Error parse.");
                tmp.push(t);
                if tmp.len() == 2 {
                    reserved_seats.push(tmp);
                    tmp = Vec::new();
                }
            }
        }
    }

    if 0 == ret || !tmp.is_empty() {
        println!("Require at least 1 parameter.");
        return;
    }

    println!(
        "Max number of families: {}",
        Solution::max_number_of_families(n, reserved_seats)
    );
}
