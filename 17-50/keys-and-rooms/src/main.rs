use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let len: usize = rooms.len();
        let mut visits: Vec<bool> = vec![false; len];
        visits[0] = true;
        let mut queue: Vec<i32> = vec![0];
        while !queue.is_empty() {
            let mut tmp: Vec<i32> = Vec::new();
            for room in queue {
                for key in &rooms[room as usize] {
                    if visits[*key as usize] {
                        continue;
                    } else {
                        tmp.push(*key);
                        visits[*key as usize] = true;
                    }
                }
            }
            queue = tmp;
        }
        for visit in &visits {
            if !visit {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut rooms: Vec<Vec<i32>> = Vec::new();
    let mut tmp: Vec<i32> = Vec::new();
    let mut count: i32 = -1;
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                if count < 0 {
                    if n == 0 {
                        rooms.push(tmp);
                        tmp = Vec::new();
                    } else {
                        count = n;
                    }
                } else {
                    tmp.push(n);
                    if tmp.len() == count as usize {
                        rooms.push(tmp);
                        tmp = Vec::new();
                        count = -1;
                    }
                }
            }
        }
    }

    if 0 == ret || count != -1 {
        println!("Require at least one parameter.");
        return;
    }

    println!("Visit all rooms: {}", Solution::can_visit_all_rooms(rooms));
}
