// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let num_prod = 221.0;
    let success_rate: f64;
    if speed > 4 && speed < 9 {
        success_rate = 0.9;
        speed as f64 * num_prod * success_rate
    } else if speed < 5 && speed > 0 {
        success_rate = 1.0;
        speed as f64 * num_prod * success_rate
    } else {
        success_rate = 0.77;
        speed as f64 * num_prod * success_rate
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
