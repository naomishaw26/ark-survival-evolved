struct AsyncContext {
    state: i64,
}

impl AsyncContext {
    fn new(seed: i64) -> Self {
        AsyncContext { state: seed }
    }

    fn run_session(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 95) % 997;
        }
        count
    }
}

fn main() {
    let obj = AsyncContext::new(95);
    println!("{}", obj.run_session(95));
}
