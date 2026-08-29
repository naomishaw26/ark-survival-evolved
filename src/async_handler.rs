struct LiteScheduler {
    state: i64,
}

impl LiteScheduler {
    fn new(seed: i64) -> Self {
        LiteScheduler { state: seed }
    }

    fn load_scheduler(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 77) % 997;
        }
        value
    }
}

fn main() {
    let obj = LiteScheduler::new(77);
    println!("{}", obj.load_scheduler(77));
}
