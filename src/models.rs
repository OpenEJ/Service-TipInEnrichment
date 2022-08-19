use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Log {
    engine_speed: u64,
    cl_ol_status: u8,
    throttle: f64,
    boost_error: f64,
    wideband_afr: f64,
    tip_in_throttle: f64,
}

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("RPM: {}, CL/OL: {}, Throttle %: {}, Boost Error: {}, Wideband AFR: {}, Tip-In-Throttle: {}", self.engine_speed, self.cl_ol_status, self.throttle, self.boost_error, self.wideband_afr, self.tip_in_throttle).fmt(f)
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
