use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Log {
    pub engine_speed: u64,
    pub cl_ol_status: u8,
    pub throttle_open_angle: f64,
    pub boost_error: f64,
    pub wideband_afr: f64,
    pub tip_in_throttle: f64,
}

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("RPM: {}, CL/OL: {}, Throttle %: {}, Boost Error: {}, Wideband AFR: {}, Tip-In-Throttle: {}", self.engine_speed, self.cl_ol_status, self.throttle_open_angle, self.boost_error, self.wideband_afr, self.tip_in_throttle).fmt(f)
    }
}

#[derive(Serialize)]
pub struct Correction {
    pub throttle_angle_change: f64,
    pub correction: f64,
    pub frequency: u64
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
