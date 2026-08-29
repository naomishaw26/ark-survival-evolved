struct SharedController {
    state: i64,
}

impl SharedController {
    fn new(seed: i64) -> Self {
        SharedController { state: seed }
    }

    fn load_provider(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 75) % 997;
        }
        result
    }
}

fn main() {
    let obj = SharedController::new(75);
    println!("{}", obj.load_provider(75));
}
