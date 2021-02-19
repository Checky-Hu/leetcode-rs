enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct NestedIterator {
    index_: usize,
    vec_: Vec<i32>,
}

impl NestedIterator {
    fn flatten_nested_list(nested_list: &[NestedInteger], result: &mut Vec<i32>) {
        for v in nested_list.iter() {
            match v {
                NestedInteger::Int(integer) => result.push(*integer),
                NestedInteger::List(list) => NestedIterator::flatten_nested_list(&list, result),
            }
        }
    }

    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut vec: Vec<i32> = Vec::new();
        NestedIterator::flatten_nested_list(&nested_list, &mut vec);
        NestedIterator {
            index_: 0,
            vec_: vec,
        }
    }

    fn next(&mut self) -> i32 {
        self.index_ += 1;
        self.vec_[self.index_ - 1]
    }

    fn has_next(&self) -> bool {
        self.index_ < self.vec_.len()
    }
}

fn main() {
    let mut obj: NestedIterator = NestedIterator::new(vec![
        NestedInteger::Int(1),
        NestedInteger::List(vec![NestedInteger::Int(2), NestedInteger::Int(3)]),
    ]);
    while obj.has_next() {
        print!("{} ", obj.next());
    }
    println!();
}
