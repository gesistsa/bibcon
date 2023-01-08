use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let md_filename = &args[1];
    let bib_filename = &args[2];
    bibcon::bibcon(md_filename, bib_filename);
}
