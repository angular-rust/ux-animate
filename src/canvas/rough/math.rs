// use rand::prelude::*;
pub struct Random {
  pub seed: u64,
}

pub fn random_seed() -> u64 {
  (rand::random::<f64>() * 2.0).powi(31).floor() as u64
}

impl Random {
  pub fn new(seed: u64) -> Self {
    Self {
      seed
    }
  }

  pub fn next(&self) -> u64 {
    // if self.seed != 0 {
    //   self.seed = 48271 * self.seed;
    //   return (((2_u64.pow(31) - 1) & self.seed) as f64 / 2.0).powi(31) as u64
    // }
    rand::random::<u64>()
  }
}