use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let md_filename = &args[1];
    let text = fs::read_to_string(md_filename)
        .expect("Should have been able to read the file");
    let keys = bibcon::extract_citekeys(&text);
    for key in &keys {
	println!("key: {key}");
    }
}
