struct ExamRoom {
    n_: i32,
    v_: Vec<i32>,
}

impl ExamRoom {
    fn new(N: i32) -> Self {
        ExamRoom {
            n_: N,
            v_: Vec::new(),
        }
    }

    fn seat(&mut self) -> i32 {
        let len: usize = self.v_.len();
        let mut position: usize = 0;
        let mut distance: i32 = 0;
        for i in 0..len {
            let (mut tmp_d, mut tmp_p) = if i == 0 {
                (self.v_[i], 0)
            } else {
                ((self.v_[i] - self.v_[i - 1]) / 2, i)
            };
            if i == len - 1 {
                let end_d: i32 = self.n_ - self.v_[len - 1] - 1;
                if end_d > tmp_d {
                    tmp_d = end_d;
                    tmp_p = len;
                }
            }
            if tmp_d > distance {
                position = tmp_p;
                distance = tmp_d;
            }
        }
        let seat: i32 = if position == 0 {
            0
        } else if position == len {
            self.n_ - 1
        } else {
            self.v_[position - 1] + (self.v_[position] - self.v_[position - 1]) / 2
        };
        self.v_.insert(position, seat);
        seat
    }

    fn leave(&mut self, p: i32) {
        let len: usize = self.v_.len();
        let mut target: usize = len;
        for (i, v) in self.v_.iter().enumerate() {
            if *v == p {
                target = i;
                break;
            }
        }
        if target != len {
            self.v_.remove(target);
        }
    }
}

fn main() {
    let mut obj: ExamRoom = ExamRoom::new(8);
    println!("seat: {}", obj.seat());
    println!("seat: {}", obj.seat());
    println!("seat: {}", obj.seat());
    obj.leave(0);
    obj.leave(7);
    println!("seat: {}", obj.seat());
    println!("seat: {}", obj.seat());
    println!("seat: {}", obj.seat());
    println!("seat: {}", obj.seat());
    println!("seat: {}", obj.seat());
    println!("seat: {}", obj.seat());
    println!("seat: {}", obj.seat());
}
