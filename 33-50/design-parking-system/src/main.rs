struct ParkingSystem {
    capacity: Vec<i32>,
    count: Vec<i32>,
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem {
            capacity: vec![big, medium, small],
            count: vec![0; 3],
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        let index: usize = (car_type - 1) as usize;
        if self.count[index] == self.capacity[index] {
            false
        } else {
            self.count[index] += 1;
            true
        }
    }
}

fn main() {
    let mut obj: ParkingSystem = ParkingSystem::new(1, 1, 0);
    println!("add car 1: {}", obj.add_car(1));
    println!("add car 2: {}", obj.add_car(2));
    println!("add car 3: {}", obj.add_car(3));
    println!("add car 1: {}", obj.add_car(1));
}
