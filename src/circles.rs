use std::f64::consts::PI;

pub const NUM_POINTS_CIRCLE: usize = 500;
pub const RADIUS_MAX: f64 = 0.2;
// returns lat array, lon array
pub fn points_on_circle(center_lat: f64, center_lon: f64, radius: f64) -> ([f64; NUM_POINTS_CIRCLE], [f64; NUM_POINTS_CIRCLE]) {
    // lat is y
    let mut lat_array: [f64; NUM_POINTS_CIRCLE] = [center_lat; NUM_POINTS_CIRCLE];
    // lon is x
    let mut lon_array: [f64; NUM_POINTS_CIRCLE] = [center_lon; NUM_POINTS_CIRCLE];
    let angle_increment = (2.0 * PI) / (NUM_POINTS_CIRCLE as f64);
    // index in array
    for i in 0..NUM_POINTS_CIRCLE{
        let theta = (i as f64) * angle_increment;
        lat_array[i] = center_lat + radius * f64::cos(theta);
        lon_array[i] = center_lon + radius * f64::sin(theta);
    }
    (lat_array, lon_array)
}