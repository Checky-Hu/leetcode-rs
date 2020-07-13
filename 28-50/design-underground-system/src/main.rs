use std::collections::HashMap;

struct UndergroundSystem {
    // <id, (station_name, t)>
    flyings: HashMap<i32, (String, i32)>,
    // <(check in, check out), (count, time)>
    arrived: HashMap<(String, String), (i32, i32)>,
}

impl UndergroundSystem {
    fn new() -> Self {
        UndergroundSystem {
            flyings: HashMap::new(),
            arrived: HashMap::new(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.flyings.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        if let Some(x) = self.flyings.remove(&id) {
            let entry: (String, String) = (x.0, station_name);
            match self.arrived.get_mut(&entry) {
                Some(y) => {
                    y.0 += 1;
                    y.1 += t - x.1;
                }
                None => {
                    self.arrived.insert(entry, (1, t - x.1));
                }
            }
        }
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let entry: (String, String) = (start_station, end_station);
        match self.arrived.get(&entry) {
            Some(x) => f64::from(x.1) / f64::from(x.0),
            None => 0_f64,
        }
    }
}

fn main() {
    let mut obj: UndergroundSystem = UndergroundSystem::new();
    obj.check_in(45, "Leyton".to_string(), 3);
    obj.check_in(32, "Paradise".to_string(), 8);
    obj.check_in(27, "Leyton".to_string(), 10);
    obj.check_out(45, "Waterloo".to_string(), 15);
    obj.check_out(27, "Waterloo".to_string(), 20);
    obj.check_out(32, "Cambridge".to_string(), 22);
    println!(
        "{}",
        obj.get_average_time("Paradise".to_string(), "Cambridge".to_string())
    );
    println!(
        "{}",
        obj.get_average_time("Leyton".to_string(), "Waterloo".to_string())
    );
    obj.check_in(10, "Leyton".to_string(), 24);
    println!(
        "{}",
        obj.get_average_time("Leyton".to_string(), "Waterloo".to_string())
    );
    obj.check_out(10, "Waterloo".to_string(), 38);
    println!(
        "{}",
        obj.get_average_time("Leyton".to_string(), "Waterloo".to_string())
    );
}
