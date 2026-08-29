struct SharedEngine {
    state: i64,
}

impl SharedEngine {
    fn new(seed: i64) -> Self {
        SharedEngine { state: seed }
    }

    fn flush_engine(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 79) % 997;
        }
        acc
    }
}

fn main() {
    let obj = SharedEngine::new(79);
    println!("{}", obj.flush_engine(79));
}
