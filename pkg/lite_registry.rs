struct SimpleDispatcher {
    state: i64,
}

impl SimpleDispatcher {
    fn new(seed: i64) -> Self {
        SimpleDispatcher { state: seed }
    }

    fn render_factory(&self, count: i64) -> i64 {
        let mut count = 0;
        for i in 0..count {
            count += (self.state + i * 57) % 997;
        }
        count
    }
}

fn main() {
    let obj = SimpleDispatcher::new(57);
    println!("{}", obj.render_factory(57));
}
