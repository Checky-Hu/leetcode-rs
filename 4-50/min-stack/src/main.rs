struct MinStack {
    stack_: Vec<i32>,
    min_: i32,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
	    stack_: Vec::new(),
	    min_: 0,
	}
    }

    fn push(&mut self, x: i32) {
        if self.stack_.is_empty() || self.min_ >= x {
	    self.stack_.push(self.min_);
	    self.min_ = x;
	}
        self.stack_.push(x);
    }

    fn pop(&mut self) {
        if !self.stack_.is_empty() {
            let tmp: i32 = self.stack_[self.stack_.len() - 1];
	    self.stack_.pop();
	    if tmp == self.min_ {
	        self.min_ = self.stack_[self.stack_.len() - 1];
	        self.stack_.pop();
	    }
	}
    }

    fn top(&mut self) -> i32 {
        if self.stack_.is_empty() {
	    0
	} else {
	    self.stack_[self.stack_.len() - 1]
	}
    }

    fn get_min(&self) -> i32 {
        self.min_
    }
}

fn main() {
    let mut obj: MinStack = MinStack::new();
    obj.push(-2);
    obj.push(0);
    obj.push(-2);
    println!("Get Min: {}", obj.get_min());
    obj.pop();
    println!("Top: {}", obj.top());
    println!("Get Min: {}", obj.get_min());
}
