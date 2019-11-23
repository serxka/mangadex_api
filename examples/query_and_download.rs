use std::env;
use std::process;

extern crate mangadex_api as md;
extern crate reqwest;

fn main () {
	let args: Vec<_> = env::args().collect();
	if args.len() != 3 { // Check that the right amount of args have been parsed
		eprintln!("Usage: mangadex_api <manga id> <chapter index>\n note: chapter index based off what was most recently uploaded");
		process::exit(1);
	}
	// Create a new anonymous session
	let session = md::Session::anon_connect ();
	// Get info about a manga
	let mut mymanga = match md::Manga::from(&session, args[1].parse::<u32>().expect("Please use a proper value")) {
		Ok(v) => v, // Handle the output
		Err(e) => {
			eprintln!("{}", e);
			process::exit(1);
		},
	};

	println!("{}", mymanga);

	// Get the chapter index from program args
	let chapter = args[2].parse::<usize>().expect("Please use a proper value");
	if chapter > mymanga.chapters.len() { // Check to see if its in range
		eprintln!("Chapter index out of range");
		process::exit(1);
	}

	println!("\n Downloading Chapter {}...", chapter);

	// Query Mangadex.org to fill the chapter with extra detail
	mymanga.chapters[chapter] = match md::Chapter::fill(&session, mymanga.chapters[chapter].id) {
		Ok(v) => v, // Handle the output
		Err(e) => {
			eprintln!("{}", e);
			process::exit(1);
		},
	};

	// Now download the chapter
	match mymanga.chapters[chapter].download(format!("./chapter{}", chapter).as_str()) {
		Err(e) => { // Handle the errors if any
			eprintln!("{}", e);
			process::exit(1);		
		},
		Ok(_) => {},
	}
}