use std::env;

mod tile;

fn main() {
    for _i in 0..5 {
        run();
    }
}

fn run() {
    let lat: f64 = 49.6;
    let lon: f64 = 15.6;
    let tiles_root = env::var("SRTM_1_SEC_FILES_ROOT").unwrap();
    let tile_name = tile::resolve_tile_name(lat, lon);
    let tile_path = tiles_root + &*tile_name;
    let file_content = tile::read_tile(&tile_path);
    let row_size = tile::compute_tile_size(&file_content);
    let elevation = tile::read_elevation(&file_content, lat, lon, row_size);
    println!("lat: {}, lon: {}, tile name: {}, elevation: {}", lat, lon, tile_name, elevation);
}