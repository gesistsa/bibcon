use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fs;

use biblatex::Bibliography;

pub fn extract_citekeys(text: &str) -> HashSet<&str>{
    lazy_static! {
        static ref HASHTAG_REGEX : Regex = Regex::new(
                r"(?P<ini>^|[\[,; -])@(?P<key>[[[:alnum:]]_\-:]+)"
            ).unwrap();
    }
    HASHTAG_REGEX.captures_iter(text).map(|c| c.name("key").unwrap().as_str()).collect()
}

fn read_bibtex(path: &str) -> Bibliography {
    let content = fs::read_to_string(&path).expect("Should have been able to read the file");
    Bibliography::parse(&content).expect("Unable to parse the main BibTeX file.")
}

fn extract_bibtex(keys: &HashSet<&str>, bib: &Bibliography) -> Bibliography {
    let mut condensed_bib = Bibliography::new();
    for key in keys {
	condensed_bib.insert(Clone::clone(bib.get(key).expect("key not found!")));
    };
    condensed_bib
}

pub fn bibcon(md_path: &str, bib_path: &str) {
    let text = fs::read_to_string(md_path)
	.expect("Should have been able to read the file");
    let keys = extract_citekeys(&text);
    let bib = read_bibtex(bib_path);
    let cbib = extract_bibtex(&keys, &bib);
    println!("{}", cbib.to_bibtex_string());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn ut_extract_citekeys_basic() {
	let text = "[@abc;@efg p. 1234] chung-hong.chan@gesis.org @hij \\@ref(fig:fig5)";
	let keys = extract_citekeys(&text);
	assert_eq!(keys.len(), 3);
	assert_eq!(keys.contains("chan"), false);
	assert_eq!(keys.contains("abc"), true);
	assert_eq!(keys.contains("efg"), true);
	assert_eq!(keys.contains("hij"), true);
    }
    #[test]
    fn ut_extract_citekeys_real1() {
	let text = fs::read_to_string("tests/weat.qmd")
	    .expect("Should have been able to read the file");
	let keys = extract_citekeys(&text);
	assert_eq!(keys.len(), 5);
	assert_eq!(keys.contains("pennington:2014:G"), true);
	assert_eq!(keys.contains("caliskan:2017:S"), true);
	assert_eq!(keys.contains("benoit:2018"), true);
	assert_eq!(keys.contains("garg:2018:W"), true);
	assert_eq!(keys.contains("chan:2022"), true);
	assert_eq!(keys.contains("gesis"), false);
    }
    #[test]
    fn ut_read_bibtext() {
	let bib = read_bibtex("tests/weat.bib");
	assert_eq!(bib.len(), 6);
    }
    #[test]
    fn ut_extract_bibtex() {
	let text = fs::read_to_string("tests/weat.qmd")
	    .expect("Should have been able to read the file");
	let keys = extract_citekeys(&text);
	let bib = read_bibtex("tests/weat.bib");
	let cbib = extract_bibtex(&keys, &bib);
	assert_eq!(bib.len(), 6);
	assert_eq!(cbib.len(), 5);
    }
}
