use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Log {
    throttle_position: f64,
    throttle_angle_change: f64,
    af_correction_short: f64,
    af_correction_learning: f64
}

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("TP: {}, TAC: {}, AF_SH: {}, AF_LR: {}", self.throttle_position, self.throttle_angle_change, self.af_correction_short, self.af_correction_learning).fmt(f)
    }
}

pub struct Correction {
    throttle_angle_change: f64,
    correction: f64,
    frequency: u64
}

impl Correction {
    pub fn new(throttle_angle_change: f64, correction: f64, frequency: u64) -> Correction {
        Correction {
            throttle_angle_change,
            correction,
            frequency
        }
    }    
}
