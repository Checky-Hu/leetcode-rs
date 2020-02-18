struct RLEIterator {
    i_: usize,
    v_: Vec<i32>,
}

impl RLEIterator {
    fn new(A: Vec<i32>) -> Self {
        RLEIterator { i_: 0, v_: A }
    }

    fn next(&mut self, n: i32) -> i32 {
        let mut t: i32 = n;
        while self.i_ < self.v_.len() {
            if self.v_[self.i_] >= t {
                self.v_[self.i_] -= t;
                return self.v_[self.i_ + 1];
            } else {
                t -= self.v_[self.i_];
                self.v_[self.i_] = 0;
                self.i_ += 2;
            }
        }
        -1
    }
}

fn main() {
    let mut obj: RLEIterator = RLEIterator::new(vec![3, 8, 0, 9, 2, 5]);
    println!("Next 2: {}", obj.next(2));
    println!("Next 1: {}", obj.next(1));
    println!("Next 1: {}", obj.next(1));
    println!("Next 2: {}", obj.next(2));
}
