#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::env;
use std::process;

mod mangadex;

fn main () {
	let args: Vec<_> = env::args().collect();
	if args.len() != 2 {
		eprintln!("Usage: mangadex_api <manga id>");
		process::exit(1);
	}
	let client: reqwest::Client = reqwest::ClientBuilder::new().build().unwrap();
	let mymanga = mangadex::manga::Manga::from(&client, args[1].parse::<u32>().expect("Please use a proper value")).unwrap();
	println!("{}", mymanga); 
}
