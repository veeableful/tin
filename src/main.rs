extern crate getopts;
extern crate iron;
extern crate staticfile;
extern crate time;

use std::env;
use std::path::Path;
use getopts::Options;
use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use staticfile::Static;
use time::precise_time_ns;

struct ResponseTime;

impl typemap::Key for ResponseTime { type Value = u64; }

impl BeforeMiddleware for ResponseTime {
	fn before(&self, req: &mut Request) -> IronResult<()> {
		req.extensions.insert::<ResponseTime>(precise_time_ns());
		Ok(())
	}
}

impl AfterMiddleware for ResponseTime {
	fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
		let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
		println!("Request took: {} ms", (delta as f64) * 0.000001);
		req.extensions.insert::<ResponseTime>(precise_time_ns());
		Ok(res)
	}
}

fn print_usage(program: &str, opts: Options) {
	let brief = format!("Usage: {} [options]", program);
	print!("{}", opts.usage(&brief));
}

fn serve_at(dir: &str, port: u16) {
	let mut chain = Chain::new(Static::new(Path::new(dir)));
	chain.link_before(ResponseTime);
	chain.link_after(ResponseTime);

	println!("Serving at localhost:{}", port);
	Iron::new(chain).http(("localhost", port)).unwrap();
}

fn main() {
	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();

	let mut opts = Options::new();
	opts.optopt("d", "directory", "directory that contains the website", "DIR");
	opts.optopt("p", "port", "server port number", "PORT");
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

	let port: String = match matches.opt_str("p") {
		Some(p) => p,
		None => "8080".to_string()
	};

	serve_at(&dir, port.parse::<u16>().unwrap());
}
