use std::{env, fs};

use crate::circles::{NUM_POINTS_CIRCLE, RADIUS_MAX};

mod tile;
mod circles;

fn main() {
    for _i in 0..5 {
        run();
    }
}

fn run() {
    let lat_center: f64 = 49.5;
    let lon_center: f64 = 14.5;
    let tiles_root = env::var("SRTM_1_SEC_FILES_ROOT").unwrap();
    let tile_name = tile::resolve_tile_name(lat_center, lon_center);
    let tile_path = tiles_root + &*tile_name;
    let file_content = tile::read_tile(&tile_path);
    let row_size = tile::compute_tile_size(&file_content);
    let mut output: String = "".to_owned();
    let mut radius: f64 = 0.002;
    let circle_increment = 0.002;
    loop {
        let points_on_circle = circles::points_on_circle(lat_center, lon_center, radius);
        for i in 0..NUM_POINTS_CIRCLE {
            let elevation = tile::read_elevation(&file_content, points_on_circle.0[i], points_on_circle.1[i], row_size);
            output = output + &format!("{},{},{}", points_on_circle.1[i], points_on_circle.0[i], elevation) + "\n";
        }
        radius = radius + circle_increment;
        if radius > RADIUS_MAX { break; }
    }
    write_string_to_file("output/output.csv", &output).expect("panic message");
}

fn write_string_to_file(path: &str, data: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(path, data)?;
    Ok(())
}
