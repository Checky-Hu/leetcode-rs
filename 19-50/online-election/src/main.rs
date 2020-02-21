struct TopVotedCandidate {
    v_: Vec<(i32, i32)>,
}

impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let len: usize = persons.len();
        let mut counts: Vec<i32> = vec![0; len + 1];
        let mut result: Vec<(i32, i32)> = Vec::with_capacity(len);
        let mut pre_winner: i32 = -1;
        for i in 0..len {
            counts[persons[i] as usize] += 1;
            if pre_winner != persons[i]
                && (pre_winner < 0 || counts[pre_winner as usize] <= counts[persons[i] as usize])
            {
                pre_winner = persons[i];
                result.push((times[i], pre_winner));
            }
        }
        TopVotedCandidate { v_: result }
    }

    fn q(&self, t: i32) -> i32 {
        let len: usize = self.v_.len();
        let mut index: usize = len;
        for (i, v) in self.v_.iter().enumerate() {
            if v.0 > t {
                index = i - 1;
                break;
            }
        }
        if index == len {
            self.v_[len - 1].1
        } else {
            self.v_[index].1
        }
    }
}

fn main() {
    let obj: TopVotedCandidate =
        TopVotedCandidate::new(vec![0, 1, 1, 0, 0, 1, 0], vec![0, 5, 10, 15, 20, 25, 30]);
    println!("Time 3: {}", obj.q(3));
    println!("Time 12: {}", obj.q(12));
    println!("Time 25: {}", obj.q(25));
    println!("Time 15: {}", obj.q(15));
    println!("Time 24: {}", obj.q(24));
    println!("Time 8: {}", obj.q(8));
}
