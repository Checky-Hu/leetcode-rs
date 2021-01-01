use std::cmp::Ordering;
use std::env;
use std::str::FromStr;

struct Solution {}

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let len: usize = students.len();
        let mut need: (usize, usize) = (0, 0);
        let mut seq0: Vec<usize> = Vec::new();
        let mut seq1: Vec<usize> = Vec::new();
        for i in 0..len {
            if students[i] == 0 {
                need.0 += 1;
            } else {
                need.1 += 1;
            }
            if sandwiches[i] == 0 {
                seq0.push(i);
            } else {
                seq1.push(i);
            }
        }
        match seq0.len().cmp(&need.0) {
            Ordering::Less => (len - seq1[need.1]) as i32,
            Ordering::Equal => 0,
            Ordering::Greater => (len - seq0[need.0]) as i32,
        }
    }
}

fn main() {
    let mut ret: usize = 0;
    let mut len: usize = 0;
    let mut students: Vec<i32> = Vec::new();
    let mut sandwiches: Vec<i32> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            1 => len = usize::from_str(&arg).expect("Error parse."),
            _ => {
                ret += 1;
                let n: i32 = i32::from_str(&arg).expect("Error parse.");
                if students.len() == len {
                    sandwiches.push(n);
                } else {
                    students.push(n);
                }
            }
        }
    }

    if 0 == ret || len == 0 || ret != len * 2 {
        println!("Require at least (1 + 2 * arg1) parameters.");
        return;
    }

    println!(
        "Students unable to eat lunch: {}",
        Solution::count_students(students, sandwiches)
    );
}
