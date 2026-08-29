struct SmartFactory {
    state: i64,
}

impl SmartFactory {
    fn new(seed: i64) -> Self {
        SmartFactory { state: seed }
    }

    fn fetch_cache(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 54) % 997;
        }
        total
    }
}

fn main() {
    let obj = SmartFactory::new(54);
    println!("{}", obj.fetch_cache(54));
}
