use co2mon::Reading;
use getopts::{Matches, Options};

pub fn time() -> u64 {
    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

pub struct Opts {
    co2: bool,
    temp: bool,
    place: String,
    name: String,
}

impl Opts {
    pub fn parse(args: &Vec<String>) -> Self {
        let mut opts = Options::new();
        Self::parse_opts(&mut opts);
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(e) => panic!("{}", e),
        };
        let co2 = matches.opt_present("c");
        let temp = matches.opt_present("t");
        let place = Self::match_opts(&matches, "p", "living");
        let name = Self::match_opts(&matches, "n", "CO2MINI");
        Opts {
            co2,
            temp,
            name,
            place,
        }
    }

    fn parse_opts(opts: &mut Options) {
        opts.optflag("c", "co2", "show co2 ppm.");
        opts.optflag("t", "temperature", "show temperature(deg. celsius).");
        opts.optopt("p", "place", "sensing place(ex: living).", "NAME");
        opts.optopt("n", "sensor-name", "sensor name(ex: CO2MINI).", "NAME");
    }

    fn match_opts(matches: &Matches, opt: &str, v: &str) -> String {
        match matches.opt_str(opt) {
            Some(val) => val,
            None => v.to_string(),
        }
    }

    pub fn show(&self, reading: &Reading, time: &str) {
        if self.co2 {
            println!("{}\t{}\t{}", self.f("co2"), reading.co2(), time);
        }
        if self.temp {
            println!("{}\t{}\t{}", self.f("temp"), reading.temperature(), time);
        };
    }

    fn f(&self, t: &str) -> String {
        format!("{}.{}.{}", self.name, t, self.place)
    }
}
