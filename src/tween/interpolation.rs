pub fn linear(v: Vec<f64>, k: f64) -> f64 {
    unimplemented!();
}

pub fn bezier(v: Vec<f64>, k: f64) -> f64 {
    unimplemented!();
}

pub fn catmull_rom(v: Vec<f64>, k: f64) -> f64 {
    unimplemented!();
}

mod utils {
    pub fn linear(p0: f64, p1: f64, t: f64) -> f64 {
        (p1 - p0) * t + p0
    }

    pub fn bernstein(n: f64, i: f64) -> f64 {
        unimplemented!()
    }

    // FIXME:
    pub fn factorial() -> f64 {
        unimplemented!()
    }

    pub fn catmull_rom(p0: f64, p1: f64, p3: f64, p4: f64, t: f64) -> f64 {
        unimplemented!()
    }
}