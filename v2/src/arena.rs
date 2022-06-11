struct DummyRng {
    state: u128,
}
impl DummyRng {
    fn new(seed: u128) -> Self {
        DummyRng { state: seed }
    }
    fn next(&mut self) -> u128 {
        let mut x = self.state;
        x ^= x << 2;
        self.state = x;
        x
    }
}

fn main() {
    let mut rng = DummyRng::new(1);
    for _ in 0..10 {
        println!("{}", rng.next());
    }
}