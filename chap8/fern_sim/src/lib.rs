//! Simulate the growth of ferns, from the level of
//! individual cells on up.

pub struct Fern {
    pub size: f64,
    pub growth_rate: f64,
}

impl Fern {
    pub fn grow(&mut self) {
        self.size *= 1.0 + self.growth_rate;
    }
}

/// Simulate the production of a spore by meiosis.
pub fn run_simulation(fern: &mut Fern, days: usize) {
    for _ in 0..days {
        fern.grow();
    }
}

/// Let the sun shine i and run the simulation
/// for a given amount of time
///
/// # use fern_sim::Terrarium;
/// # use std::time::Duration;
/// # let mut tm = Terrarium::new();
/// tm.apply_sunlight(Duration::from_secs(60));
///
pub fn apply_sunlight(&mut self, time: std::time::Duration) {}


