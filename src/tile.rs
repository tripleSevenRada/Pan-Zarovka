use std::fs;

fn lat_to_i(lat: f64) -> u8 {
    let mut lat_i = lat.abs() as u8;
    // TODO
    // update for south hemisphere
    if lat < 0.0 && lat % 1.0 != 0.0 {
        lat_i = lat_i - 1;
    }
    lat_i
}

fn lon_to_i(lon: f64) -> u8 {
    let mut lon_i = lon.abs() as u8;
    // TODO
    // update for west hemisphere
    if lon < 0.0 && lon % 1.0 != 0.0 {
        lon_i = lon_i - 1;
    }
    lon_i
}

pub fn resolve_tile_name(lat: f64, lon: f64) -> String {
    // example: "N49E014.hgt"
    let lat_i = lat_to_i(lat);
    let lon_i = lon_to_i(lon);
    let mut result = "".to_owned();
    if lat > 0.0 {
        char(&mut result, 'N');
    } else {
        char(&mut result, 'S');
    }
    if lat_i < 10 {
        zero(&mut result);
    }
    result = result + &*lat_i.to_string();
    if lon > 0.0 {
        char(&mut result, 'E');
    } else {
        char(&mut result, 'W');
    }
    if lon_i < 100 {
        zero(&mut result);
    }
    if lon_i < 10 {
        zero(&mut result);
    }
    result = result + &*lon_i.to_string() + (".hgt");
    result
}

fn zero(value: &mut String) { value.push('0'); }

fn char(value: &mut String, ch: char) {
    value.push(ch);
}

pub fn read_tile(file: &String) -> Vec<u8> { fs::read(file).unwrap() }

pub fn compute_tile_size(tile_vec: &Vec<u8>) -> i16 {
    let size = tile_vec.len();
    ((size / 2) as f64).sqrt() as i16
}

pub fn read_elevation(tile_vec: &Vec<u8>, lat: f64, lon: f64, size: i16) -> i16 {
    let lat_diff = 1.0 - (lat - lat.floor());
    let lon_diff = lon - lon.floor();
    let grid = size - 1;
    let row = (lat_diff * grid as f64) as i64;
    let col = (lon_diff * grid as f64) as i64;
    let start = (((size * 2) as i64) * row + col * 2) as usize;
    let first = tile_vec[start];
    let second = tile_vec[start + 1];
    let elevation = ((first as u16) << 8 | second as u16) as i16;
    return elevation;
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_many_corner_cases() {
        let lines = read_lines("src/resources/test-resolve-tile-name.csv");
        println!("lines (test cases): {}", lines.len());
        for line in lines {
            let v: Vec<&str> = line.split(",").collect();
            assert_eq!(v.len(), 3);
            let tuple: (f64, f64, &str) = (
                v[0].trim().parse::<f64>().unwrap(),
                v[1].trim().parse::<f64>().unwrap(),
                v[2].trim());
            assert_eq!(tuple.2, resolve_tile_name(tuple.0, tuple.1));
        }
    }

    fn read_lines(filename: &str) -> Vec<String> {
        read_to_string(filename)
            .unwrap()  // panic on possible file-reading errors
            .lines()  // split the string into an iterator of string slices
            .map(String::from)  // make each slice into a string
            .collect()  // gather them together into a vector
    }
}

