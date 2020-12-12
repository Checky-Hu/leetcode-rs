struct OrderedStream {
    len_: usize,
    // (has been returned, string value)
    vec_: Vec<(bool, String)>,
}

impl OrderedStream {
    fn new(n: i32) -> Self {
        OrderedStream {
            len_: n as usize,
            vec_: vec![(false, String::new()); n as usize],
        }
    }

    fn insert(&mut self, id: i32, value: String) -> Vec<String> {
        let i: usize = (id - 1) as usize;
        self.vec_[i].1 = value;
        let mut result: Vec<String> = Vec::new();
        if i == 0 || self.vec_[i - 1].0 {
            for v in self.vec_.iter_mut().take(self.len_).skip(i) {
                if v.1.is_empty() {
                    break;
                } else {
                    v.0 = true;
                    result.push(v.1.clone());
                }
            }
        }
        result
    }
}

fn show(v: Vec<String>) {
    for s in v.iter() {
        print!("{} ", *s);
    }
    println!();
}

fn main() {
    let mut obj: OrderedStream = OrderedStream::new(5);
    show(obj.insert(3, "ccccc".to_string()));
    show(obj.insert(1, "aaaaa".to_string()));
    show(obj.insert(2, "bbbbb".to_string()));
    show(obj.insert(5, "eeeee".to_string()));
    show(obj.insert(4, "ddddd".to_string()));
}
