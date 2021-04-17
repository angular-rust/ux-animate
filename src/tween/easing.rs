#![allow(clippy::float_cmp)]

pub mod linear {
    pub fn none_easing(amount: f64) -> f64 {
        amount
    }
}

pub mod quadratic {
    pub fn in_easing(amount: f64) -> f64 {
        amount * amount
    }

    pub fn out_easing(amount: f64) -> f64 {
        amount * (2.0 - amount)
    }

    pub fn in_out_easing(amount: f64) -> f64 {
        let mut amount = amount * 2.0;
        if amount < 1.0 {
            return 0.5 * amount * amount;
        }
        amount -= 1.0;
        -0.5 * (amount * (amount - 2.0) - 1.0)
    }
}

pub mod cubic {
    pub fn in_easing(amount: f64) -> f64 {
        amount * amount * amount
    }

    pub fn out_easing(amount: f64) -> f64 {
        let amount = amount - 1.0;
        amount * amount * amount + 1.0
    }

    pub fn in_out_easing(amount: f64) -> f64 {
        let mut amount = amount * 2.0;
        if amount < 1.0 {
            return 0.5 * amount * amount * amount;
        }
        amount -= 2.0;
        0.5 * (amount * amount * amount + 2.0)
    }
}

pub mod quartic {
    pub fn in_easing(amount: f64) -> f64 {
        amount * amount * amount * amount
    }

    pub fn out_easing(amount: f64) -> f64 {
        let amount = amount - 1.0;
        1.0 - amount * amount * amount * amount
    }

    pub fn in_out_easing(amount: f64) -> f64 {
        let mut amount = amount * 2.0;
        if amount < 1.0 {
            return 0.5 * amount * amount * amount * amount;
        }
        amount -= 2.0;
        -0.5 * (amount * amount * amount * amount - 2.0)
    }
}

pub mod quintic {
    pub fn in_easing(amount: f64) -> f64 {
        amount * amount * amount * amount * amount
    }

    pub fn out_easing(amount: f64) -> f64 {
        let amount = amount - 1.0;
        amount * amount * amount * amount * amount + 1.0
    }

    pub fn in_out_easing(amount: f64) -> f64 {
        let mut amount = amount * 2.0;
        if amount < 1.0 {
            return 0.5 * amount * amount * amount * amount * amount;
        }
        amount -= 2.0;
        0.5 * (amount * amount * amount * amount * amount + 2.0)
    }
}

pub mod sinusoidal {
    use std::f64::consts::PI;

    pub fn in_easing(amount: f64) -> f64 {
        1.0 - f64::cos(amount * PI / 2.0)
    }

    pub fn out_easing(amount: f64) -> f64 {
        f64::sin(amount * PI / 2.0)
    }

    pub fn in_out_easing(amount: f64) -> f64 {
        0.5 * (1.0 - f64::cos(amount * PI))
    }
}

pub mod exponential {
    pub fn in_easing(amount: f64) -> f64 {
        if amount == 0.0 {
            return 0.0;
        }
        f64::powf(1024.0, amount - 1.0)
    }

    pub fn out_easing(amount: f64) -> f64 {
        if amount == 1.0 {
            return 1.0;
        }
        1.0 - f64::powf(2.0, -10.0 * amount)
    }

    pub fn in_out_easing(amount: f64) -> f64 {
        if amount == 0.0 {
            return 0.0;
        }

        if amount == 1.0 {
            return 1.0;
        }

        let amount = amount * 2.0;
        if amount < 1.0 {
            return 0.5 * f64::powf(1024.0, amount - 1.0);
        }

        0.5 * (f64::powf(2.0, -10.0 * (amount - 1.0)) + 2.0)
    }
}

pub mod circular {
    pub fn in_easing(amount: f64) -> f64 {
        1.0 - f64::sqrt(1.0 - amount * amount)
    }

    pub fn out_easing(amount: f64) -> f64 {
        let amount = amount - 1.0;
        f64::sqrt(1.0 - amount * amount)
    }

    pub fn in_out_easing(amount: f64) -> f64 {
        let mut amount = amount * 2.0;
        if amount < 1.0 {
            return -0.5 * (f64::sqrt(1.0 - amount * amount) - 1.0);
        }
        amount -= 2.0;
        0.5 * (f64::sqrt(1.0 - amount * amount) + 1.0)
    }
}

pub mod elastic {
    use std::f64::consts::PI;

    pub fn in_easing(amount: f64) -> f64 {
        if amount == 0.0 {
            return 0.0;
        }

        if amount == 1.0 {
            return 1.0;
        }
        -f64::powf(2.0, 10.0 * (amount - 1.0)) * f64::sin((amount - 1.1) * 5.0 * PI)
    }

    pub fn out_easing(amount: f64) -> f64 {
        if amount == 0.0 {
            return 0.0;
        }

        if amount == 1.0 {
            return 1.0;
        }
        f64::powf(2.0, -10.0 * amount) * f64::sin((amount - 0.1) * 5.0 * PI) + 1.0
    }

    pub fn in_out_easing(amount: f64) -> f64 {
        if amount == 0.0 {
            return 0.0;
        }

        if amount == 1.0 {
            return 1.0;
        }

        let amount = amount * 2.0;
        if amount < 1.0 {
            return -0.5
                * f64::powf(2.0, 10.0 * (amount - 1.0))
                * f64::sin((amount - 1.1) * 5.0 * PI);
        }
        0.5 * f64::powf(2.0, -10.0 * (amount - 1.0)) * f64::sin((amount - 1.1) * 5.0 * PI) + 1.0
    }
}

pub mod back {
    pub fn in_easing(amount: f64) -> f64 {
        let s: f64 = 1.70158;
        amount * amount * (s + 1.0) * amount - s
    }

    pub fn out_easing(amount: f64) -> f64 {
        let s: f64 = 1.70158;
        let amount = amount - 1.0;
        amount * amount * ((s + 1.0) * amount + s) + 1.0
    }

    pub fn in_out_easing(amount: f64) -> f64 {
        let s: f64 = 1.70158 * 1.525;
        let mut amount = amount * 2.0;
        if amount < 1.0 {
            return 0.5 * (amount * amount * ((s + 1.0) * amount - s));
        }
        amount -= 2.0;
        0.5 * (amount * amount * ((s + 1.0) * amount + s) + 2.0)
    }
}

pub mod bounce {
    pub fn in_easing(amount: f64) -> f64 {
        1.0 - out_easing(1.0 - amount)
    }

    pub fn out_easing(amount: f64) -> f64 {
        if amount < 1.0 / 2.75 {
            return 7.5625 * amount * amount;
        } else if amount < 2.0 / 2.75 {
            let amount = amount - 1.5 / 2.75;
            return 7.5625 * amount * amount + 0.75;
        } else if amount < 2.5 / 2.75 {
            let amount = amount - 2.25 / 2.75;
            return 7.5625 * amount * amount + 0.9375;
        }
        let amount = amount - 2.625 / 2.75;
        7.5625 * amount * amount + 0.984375
    }

    pub fn in_out_easing(amount: f64) -> f64 {
        if amount < 0.5 {
            return in_easing(amount * 2.0) * 0.5;
        }
        out_easing(amount * 2.0 - 1.0) * 0.5 + 0.5
    }
}
