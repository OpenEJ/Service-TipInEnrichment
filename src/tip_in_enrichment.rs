use crate::models::Log;
use crate::models::Correction;

pub fn begin(data: Vec<Log>) -> Vec<Correction>{
    println!("Beginning Calcs. Data Length: {}", data.len());
    let without_open_loop = remove_open_loop(data);
    println!("Removed Open Loop Data. Data Length: {}", without_open_loop.len());
    let without_high_boost_error = remove_high_boost_error(without_open_loop);
    println!("Removed High Boost Error Data. Data Length: {}", without_high_boost_error.len());
    let correction_tables = build_correction_table(without_high_boost_error);
    println!("Build Correction Tables");
    correction_tables
}

fn remove_open_loop(mut data: Vec<Log>) -> Vec<Log> {
    for i in 0..data.len() {
        if data[i].cl_ol_status == 10 {
            data.remove(i);
        }
        if i + 1 >= data.len() {
            break;
        }
    }
    data
}

fn remove_high_boost_error(mut data: Vec<Log>) -> Vec<Log> {
    for i in 0..data.len() {
        if data[i].boost_error > 8.0 {
            data.remove(i);
        }
        if i + 1 >= data.len() {
            break;
        }
    }
    data
}

fn generate_blank_correction_table() -> Vec<Correction> {
    let tac_value = vec![0.4, 2.8, 5.2, 7.5, 9.9, 12.3, 14.6, 17.0, 19.4, 21.7, 24.1, 26.5, 28.8, 31.2, 33.6, 35.9, 38.3, 40.7];
    let mut corrections : Vec<Correction> =  Vec::new();
    for value in tac_value {
        corrections.push(Correction::new(value, 0.0, 0));
    }
    corrections
}

fn build_correction_table(data: Vec<Log>) -> Vec<Correction> {
    let mut corrections = generate_blank_correction_table();
    for i in data {
        let error = (i.wideband_afr - 14.7) / 14.7;
        let corr_factor = (1.0 - error) / 2.0;
        for j in &mut corrections {
            if i.tip_in_throttle < j.throttle_angle_change {
                j.correction = ( ( j.correction * j.frequency as f64) + corr_factor ) / ( j.frequency + 1) as f64;
                j.frequency += 1;
                break;
            }
        }
    }
    corrections
}
