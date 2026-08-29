struct SimpleScheduler {
    state: i64,
}

impl SimpleScheduler {
    fn new(seed: i64) -> Self {
        SimpleScheduler { state: seed }
    }

    fn collect_client(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 85) % 997;
        }
        count
    }
}

fn main() {
    let obj = SimpleScheduler::new(85);
    println!("{}", obj.collect_client(85));
}
