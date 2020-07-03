use std::cmp::Ordering;
use std::env;

struct Solution {}

impl Solution {
    pub fn rank_teams(votes: Vec<String>) -> String {
        let rank: usize = votes[0].len();
        let mut teams: Vec<(char, Vec<i32>)> = Vec::with_capacity(26);
        for i in 0..26 {
            teams.push(((i as u8 + 65) as char, vec![0; rank]));
        }
        for vote in votes.iter() {
            for (i, c) in vote.chars().enumerate() {
                teams[c as usize - 65].1[i] += 1;
            }
        }
        teams.sort_by(|a, b| {
            for i in 0..rank {
                match a.1[i].cmp(&b.1[i]) {
                    Ordering::Greater => return Ordering::Less,
                    Ordering::Equal => continue,
                    Ordering::Less => return Ordering::Greater,
                }
            }
            Ordering::Equal
        });
        let mut result: String = String::new();
        for v in teams.iter().take(rank) {
            result.push(v.0);
        }
        result
    }
}

fn main() {
    let mut ret: i32 = 0;
    let mut votes: Vec<String> = Vec::new();
    for (index, arg) in env::args().enumerate() {
        match index {
            0 => (),
            _ => {
                ret += 1;
                let s: String = arg;
                votes.push(s);
            }
        }
    }

    if 0 == ret {
        println!("Require at least one parameter.");
    }
    println!("Rank teams: {}", Solution::rank_teams(votes));
}
