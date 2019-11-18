use std::env;
use std::process;

extern crate mangadex_api as mangadex;
extern crate reqwest;

fn main () {
	let args: Vec<_> = env::args().collect();
	if args.len() != 2 {
		eprintln!("Usage: mangadex_api <manga id>");
		process::exit(1);
	}
	let client: reqwest::Client = reqwest::ClientBuilder::new().build().unwrap();
	let mymanga = mangadex::manga::Manga::from(&client, args[1].parse::<u32>().expect("Please use a proper value"));
	let mymanga = match mymanga {
		Ok(v) => v,
		Err(e) => {
			eprintln!("{}", e);
			process::exit(1);
		},
	};
	println!("{}", mymanga); 
}