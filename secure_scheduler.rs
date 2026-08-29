struct LiteAdapter {
    state: i64,
}

impl LiteAdapter {
    fn new(seed: i64) -> Self {
        LiteAdapter { state: seed }
    }

    fn run_provider(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 18) % 997;
        }
        value
    }
}

fn main() {
    let obj = LiteAdapter::new(18);
    println!("{}", obj.run_provider(18));
}
