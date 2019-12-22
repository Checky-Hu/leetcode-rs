struct MyCircularQueue {
    start_: usize,
    count_: usize,
    len_: usize,
    v_: Vec<i32>,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        MyCircularQueue {
            start_: 0,
            count_: 0,
            len_: k as usize,
            v_: vec![-1; k as usize],
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.count_ == self.len_ {
            false
        } else {
            self.v_[(self.start_ + self.count_) % self.len_] = value;
            self.count_ += 1;
            true
        }
    }

    fn de_queue(&mut self) -> bool {
        if self.count_ == 0 {
            false
        } else {
            self.start_ = (self.start_ + 1) % self.len_;
            self.count_ -= 1;
            true
        }
    }

    fn front(&self) -> i32 {
        if self.count_ == 0 {
            -1
        } else {
            self.v_[self.start_]
        }
    }

    fn rear(&self) -> i32 {
        if self.count_ == 0 {
            -1
        } else {
            self.v_[(self.start_ + self.count_ - 1) % self.len_]
        }
    }

    fn is_empty(&self) -> bool {
        self.count_ == 0
    }

    fn is_full(&self) -> bool {
        self.count_ == self.len_
    }
}

fn main() {
    let mut obj: MyCircularQueue = MyCircularQueue::new(3);
    println!("is_empty: {}", obj.is_empty());
    println!("en_queue 1: {}", obj.en_queue(1));
    println!("is_empty: {}", obj.is_empty());
    println!("en_queue 2: {}", obj.en_queue(2));
    println!("en_queue 3: {}", obj.en_queue(3));
    println!("en_queue 4: {}", obj.en_queue(4));
    println!("rear: {}", obj.rear());
    println!("is_full: {}", obj.is_full());
    println!("de_queue: {}", obj.de_queue());
    println!("en_queue 4: {}", obj.en_queue(4));
    println!("front: {}", obj.front());
    println!("rear: {}", obj.rear());
}
