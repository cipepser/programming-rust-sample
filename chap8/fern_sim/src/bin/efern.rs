extern crate fern_sim;
use fern_sim::{Fern, run_simulation};

fn main() {
    let mut fern = Fern {
        size: 1.0,
        growth_rate: 0.0001,
    };

    run_simulation(&mut fern, 1000);
    println!("final fernn size: {}", fern.size);
}
