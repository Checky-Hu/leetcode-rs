struct CombinationIterator {
    v_: Vec<String>,
    i_: usize,
}

impl CombinationIterator {
    fn new(characters: String, combinationLength: i32) -> Self {
        let mut current: String = String::new();
        let mut result: Vec<String> = Vec::new();
        CombinationIterator::generate(
            characters.as_bytes(),
            combinationLength as usize,
            0,
            &mut current,
            &mut result,
        );
        CombinationIterator { v_: result, i_: 0 }
    }

    fn generate(
        characters: &[u8],
        length: usize,
        index: usize,
        current: &mut String,
        result: &mut Vec<String>,
    ) {
        if current.len() == length {
            result.push(current.clone());
        } else {
            for i in index..characters.len() {
                current.push(characters[i] as char);
                CombinationIterator::generate(characters, length, i + 1, current, result);
                current.pop();
            }
        }
    }

    fn next(&mut self) -> String {
        let result: String = self.v_[self.i_].clone();
        self.i_ += 1;
        result
    }

    fn has_next(&self) -> bool {
        self.i_ < self.v_.len()
    }
}

fn main() {
    let mut obj: CombinationIterator = CombinationIterator::new("abc".to_string(), 2);
    println!("next: {}", obj.next());
    println!("has_next: {}", obj.has_next());
    println!("next: {}", obj.next());
    println!("has_next: {}", obj.has_next());
    println!("next: {}", obj.next());
    println!("has_next: {}", obj.has_next());
}
