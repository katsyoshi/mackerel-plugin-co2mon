extern crate getopts;
pub mod utils;

use co2mon::{Result, Sensor};

fn main() -> Result<()> {
    let sensor = Sensor::open_default()?;
    let reading = sensor.read()?;
    let time = utils::time();
    let opts = utils::Opts::parse(&std::env::args().collect::<Vec<String>>());

    opts.show(&reading, &time.to_string());
    Ok(())
}
