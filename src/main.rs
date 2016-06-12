extern crate getopts;
extern crate iron;
extern crate staticfile;

use std::env;
use std::path::Path;
use getopts::Options;
use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use staticfile::Static;
use std::time::Instant;

struct ResponseTime;

impl typemap::Key for ResponseTime { type Value = Instant; }

impl BeforeMiddleware for ResponseTime {
	fn before(&self, req: &mut Request) -> IronResult<()> {
		req.extensions.insert::<ResponseTime>(Instant::now());
		Ok(())
	}
}

impl AfterMiddleware for ResponseTime {
	fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
		let delta = req.extensions.get::<ResponseTime>().unwrap().elapsed();
		let secs = delta.as_secs() as u64;
		let nanos = delta.subsec_nanos() as u64;
		let elapsed_ns = secs * 1_000_000_000 + nanos;
		let (elapsed, unit) = to_number_and_unit(elapsed_ns);
		println!("{} /{} took: {:.3} {}", req.method, req.url.path.join("/"), elapsed, unit);
		Ok(res)
	}
}

static UNITS: [&'static str; 4] = [ "ns", "μs", "ms", "s" ];

fn to_number_and_unit(mut elapsed: u64) -> (u64, &'static str) {
    let mut unit_index = 0;

    while elapsed >= 1_000 {
        elapsed /= 1_000;
        if unit_index < UNITS.len() - 1 {
            unit_index += 1;
        }
    }

    (elapsed, &UNITS[unit_index])
}

fn print_usage(program: &str, opts: Options) {
	let brief = format!("Usage: {} [options]", program);
	print!("{}", opts.usage(&brief));
}

fn serve_at(dir: &str, port: u16, time_response: bool) {
	let static_handler = Static::new(Path::new(dir));
	println!("Serving at 0.0.0.0:{}", port);

	if time_response {
		let mut chain = Chain::new(static_handler);
		chain.link_before(ResponseTime);
		chain.link_after(ResponseTime);
		Iron::new(chain).http(("0.0.0.0", port)).unwrap();
	} else {
		Iron::new(static_handler).http(("0.0.0.0", port)).unwrap();
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();

	let mut opts = Options::new();
	opts.optopt("d", "directory", "directory that contains the website", "DIR");
	opts.optopt("p", "port", "server port number", "PORT");
	opts.optopt("t", "time", "should server time responses?", "true/false");
	opts.optflag("h", "help", "print this help menu");
	let matches = match opts.parse(&args[1..]) {
		Ok(m) => { m }
		Err(f) => { panic!(f.to_string()) }
	};
	if matches.opt_present("h") {
		print_usage(&program, opts);
		return;
	}

	let dir: String = match matches.opt_str("d") {
		Some(d) => d,
		None => ".".to_string()
	};

	let port: u16 = match matches.opt_str("p") {
		Some(p) => p.parse::<u16>().unwrap(),
		None => 8080
	};

	let time_response: bool = match matches.opt_str("t") {
		Some(p) => !(p == "false"), // prefer 'true' unless 'false' is spelled correctly
		None => true
	};

	serve_at(&dir, port, time_response);
}
