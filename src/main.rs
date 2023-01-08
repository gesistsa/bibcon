use lazy_static::lazy_static;

use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;

fn extract_citekeys(text: &str) -> HashSet<&str>{
    lazy_static! {
        static ref HASHTAG_REGEX : Regex = Regex::new(
                r"(?P<ini>|[\[,; -])@(?P<key>[[[:alnum:]]_\-:]+)"
            ).unwrap();
    }
    HASHTAG_REGEX.captures_iter(text).map(|c| c.name("key").unwrap().as_str()).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let md_filename = &args[1];
    let text = fs::read_to_string(md_filename)
        .expect("Should have been able to read the file");
    let keys = extract_citekeys(&text);
    for key in &keys {
	println!("key: {key}");
    }
}
