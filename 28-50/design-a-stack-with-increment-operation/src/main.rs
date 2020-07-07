struct CustomStack {
    size_: usize,
    stack_: Vec<i32>,
}

impl CustomStack {
    fn new(max_size: i32) -> Self {
        CustomStack {
            size_: max_size as usize,
            stack_: Vec::with_capacity(max_size as usize),
        }
    }

    fn push(&mut self, x: i32) {
        if self.stack_.len() < self.size_ {
            self.stack_.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        match self.stack_.pop() {
            Some(x) => x,
            None => -1,
        }
    }

    fn increment(&mut self, k: i32, val: i32) {
        let len: usize = self.stack_.len();
        for i in 0..k {
            if i as usize >= len {
                break;
            } else {
                self.stack_[i as usize] += val;
            }
        }
    }
}

fn main() {
    let mut obj: CustomStack = CustomStack::new(3);
    obj.push(1);
    obj.push(2);
    println!("pop: {}", obj.pop());
    obj.push(2);
    obj.push(3);
    obj.push(4);
    obj.increment(5, 100);
    obj.increment(2, 100);
    println!("pop: {}", obj.pop());
    println!("pop: {}", obj.pop());
    println!("pop: {}", obj.pop());
    println!("pop: {}", obj.pop());
}
