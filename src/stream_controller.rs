struct StreamContext {
    state: i64,
}

impl StreamContext {
    fn new(seed: i64) -> Self {
        StreamContext { state: seed }
    }

    fn run_collector(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 91) % 997;
        }
        acc
    }
}

fn main() {
    let obj = StreamContext::new(91);
    println!("{}", obj.run_collector(91));
}
