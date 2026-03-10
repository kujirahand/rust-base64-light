use base64_light::{base64_decode_str, base64_encode};
use std::env;
use std::process;

fn print_usage(program: &str) {
	eprintln!("Usage: {program} <e|d|encode|decode> <str>");
	eprintln!("  e <str>  Encode string to base64");
	eprintln!("  d <str>  Decode base64 to string");
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 3 {
		print_usage(&args[0]);
		process::exit(1);
	}

	let mode = &args[1];
	let input = args[2..].join(" ");

	match mode.as_str() {
		"e" => println!("{}", base64_encode(&input)),
		"d" => println!("{}", base64_decode_str(&input)),
		"encode" => println!("{}", base64_encode(&input)),
		"decode" => println!("{}", base64_decode_str(&input)),
        "table" => println!("{}", base64_light::base64_table_printer()),
		_ => {
			eprintln!("Unknown mode: {mode}");
			print_usage(&args[0]);
			process::exit(1);
		}
	}
}
