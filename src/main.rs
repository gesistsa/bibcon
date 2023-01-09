use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let md_filename = &args[1];
    let bib_filename = &args[2];
    bibcon::bibcon(vec![md_filename], bib_filename);
}
