mod gyro;
use gyro::Gyroscope;

fn main() {
    match Gyroscope::new() {
        Ok(gyroscope) => {
            println!("Gyroscope X Raw: {}", gyroscope.x_raw);
            println!("Gyroscope Y Raw: {}", gyroscope.y_raw);
            println!("Gyroscope Z Raw: {}", gyroscope.z_raw);
        }
        Err(err) => eprintln!("Error initializing gyroscope: {}", err),
    }
}
