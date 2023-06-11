use std::fs::File;
use std::io::{BufRead, BufReader};

const GYROSCOPE_X_RAW: &str = "/sys/bus/iio/devices/iio:device1/in_anglvel_x_raw";
const GYROSCOPE_Y_RAW: &str = "/sys/bus/iio/devices/iio:device1/in_anglvel_y_raw";
const GYROSCOPE_Z_RAW: &str = "/sys/bus/iio/devices/iio:device1/in_anglvel_z_raw";

pub struct Gyroscope {
    pub x_raw: i32,
    pub y_raw: i32,
    pub z_raw: i32,
}

impl Gyroscope {
    pub fn new() -> Result<Self, std::io::Error> {
        Ok(Self {
            x_raw: read_gyroscope_value(GYROSCOPE_X_RAW)?,
            y_raw: read_gyroscope_value(GYROSCOPE_Y_RAW)?,
            z_raw: read_gyroscope_value(GYROSCOPE_Z_RAW)?,
        })
    }
}

fn read_gyroscope_value(file_path: &str) -> Result<i32, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap()?;
    let value = line.trim().parse::<i32>().unwrap();
    Ok(value)
}
