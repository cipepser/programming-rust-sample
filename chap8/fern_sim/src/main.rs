struct Fern {
    size: f64,
    growth_rate: f64,
}

impl Fern {
    fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }
}

fn run_simulation(fern: &mut Fern, days: usize) {
    for _ in 0..days {
        fern.grow();
    }
}


fn main() {
    let mut fern = Fern {
        size: 1.0,
        growth_rate: 0.0001,
    };

    run_simulation(&mut fern, 1000);
    println!("final fernn size: {}", fern.size);
}
