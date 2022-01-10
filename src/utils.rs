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
        let (co2, temp) = Self::default_flag(&matches);
        Opts {
            co2,
            temp,
            name: Self::match_opts(&matches, "n", "CO2MINI"),
            place: Self::match_opts(&matches, "p", "living"),
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

    fn default_flag(m: &Matches) -> (bool, bool) {
        let (c, t) = (m.opt_present("c"), m.opt_present("t"));

        if c == t {
            (true, true)
        } else {
            (c, t)
        }
    }

    pub fn show(&self, reading: &Reading, time: &str) {
        if self.co2 {
            println!("{}\t{}\t{}", self.f("co2"), reading.co2(), time);
        }
        if self.temp {
            println!("{}\t{}\t{}", self.f("temp"), reading.temperature(), time);
        }
    }

    fn f(&self, t: &str) -> String {
        format!("{}.{}.{}", self.name, t, self.place)
    }
}

#[test]
fn default_opts() {
    let opts = Opts::parse(&vec!["".to_string(), "".to_string()]);
    assert_eq!(opts.co2, true);
    assert_eq!(opts.temp, true);
    assert_eq!(opts.name, String::from("CO2MINI"));
    assert_eq!(opts.place, String::from("living"));
}

#[test]
fn t() {
    let opts = Opts::parse(&vec!["".to_string(), "-t".to_string()]);
    assert_eq!(opts.co2, false);
    assert_eq!(opts.temp, true);
}

#[test]
fn n() {
    let opts = Opts::parse(&vec!["".to_string(), "-n".to_string(), "hoge".to_string()]);
    assert_eq!(&opts.name, "hoge");
}
