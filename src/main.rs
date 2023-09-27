use clap::{Arg, App};

fn main() {
    let matches = App::new("BibTeX Condenser").
	version("0.0.4").
	author("Chung-hong Chan <chainsawtiney@gmail.com>").
	arg(Arg::with_name("bib").short("b").long("bib").value_name("BIBFILE").help("Main BibTeX file").takes_value(true)).
	arg(Arg::with_name("outbib").short("o").long("outbib").value_name("OUTBIBFILE").help("Output BibTeX file").takes_value(true)).
	arg(Arg::with_name("md").value_name("MDFILES").help("Markdown file(s)").required(true).min_values(1)).
	get_matches();
    let bib_filename = matches.value_of("bib").unwrap();
    let md_filename: Vec<_> = matches.values_of("md").unwrap().collect();
    let out_filename = matches.value_of("outbib");
    bibcon::bibcon(md_filename, bib_filename, out_filename);
}
