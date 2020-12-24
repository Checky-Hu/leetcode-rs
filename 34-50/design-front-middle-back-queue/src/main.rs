struct FrontMiddleBackQueue {
    len_: usize,
    vec_: Vec<i32>,
}

impl FrontMiddleBackQueue {
    fn new() -> Self {
        FrontMiddleBackQueue {
            len_: 0,
            vec_: Vec::with_capacity(100),
        }
    }

    fn push_front(&mut self, val: i32) {
        self.len_ += 1;
        self.vec_.insert(0, val);
    }

    fn push_middle(&mut self, val: i32) {
        self.vec_.insert(self.len_ >> 1, val);
        self.len_ += 1;
    }

    fn push_back(&mut self, val: i32) {
        self.len_ += 1;
        self.vec_.push(val);
    }

    fn pop_front(&mut self) -> i32 {
        if self.len_ == 0 {
            -1
        } else {
            self.len_ -= 1;
            self.vec_.remove(0)
        }
    }

    fn pop_middle(&mut self) -> i32 {
        if self.len_ == 0 {
            -1
        } else {
            self.len_ -= 1;
            self.vec_.remove(self.len_ >> 1)
        }
    }

    fn pop_back(&mut self) -> i32 {
        if let Some(x) = self.vec_.pop() {
            self.len_ -= 1;
            x
        } else {
            -1
        }
    }
}

fn main() {
    let mut obj: FrontMiddleBackQueue = FrontMiddleBackQueue::new();
    obj.push_front(1);
    obj.push_back(2);
    obj.push_middle(3);
    obj.push_middle(4);
    println!("pop_front: {}", obj.pop_front());
    println!("pop_middle: {}", obj.pop_middle());
    println!("pop_middle: {}", obj.pop_middle());
    println!("pop_back: {}", obj.pop_back());
    println!("pop_front: {}", obj.pop_front());
}
