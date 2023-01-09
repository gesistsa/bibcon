use clap::{Arg, App};

fn main() {
    let matches = App::new("BibTeX Condenser").
	version("0.0.1").
	author("Chung-hong Chan").
	arg(Arg::with_name("bib").short("b").long("bib").value_name("BIBFILE").help("Main BibTeX file").takes_value(true)).
	arg(Arg::with_name("md").value_name("mdfiles").help("Multiple Markdown files").min_values(1)).
	get_matches();
    let bib_filename = matches.value_of("bib").unwrap();
    let md_filename: Vec<_> = matches.values_of("md").unwrap().collect();
    bibcon::bibcon(md_filename, bib_filename);
}
