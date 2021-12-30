use co2mon::{Result, Sensor};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> Result<()> {
    let sensor = Sensor::open_default()?;
    let reading = sensor.read()?;
    let time = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };
    let co2_metric_name = format!("{}.{}.{}", "CO2MINI", "co2", "living");
    let temp_metric_name = format!("{}.{}.{}", "CO2MINI", "temp", "living");
    println!("{}\t{}\t{}", co2_metric_name, reading.co2(), time.to_string());
    println!("{}\t{}\t{}", temp_metric_name, reading.temperature(), time.to_string());
    Ok(())
}
