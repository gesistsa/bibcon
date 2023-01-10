use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::process;

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
    let bib_content = fs::read_to_string(&path);
    match bib_content {
	Ok(content) => {
	    Bibliography::parse(&content).expect("Unable to parse the main BibTeX file.")
	},
	Err(_err) => {
	    eprint!("Cannot read BibTeX file: {}.\n", &path);
	    process::exit(1);
	}
    }
}

fn extract_bibtex(keys: &Vec<&str>, bib: &Bibliography) -> Bibliography {
    let mut condensed_bib = Bibliography::new();
    for key in keys {
	let tempbib = bib.get(&key); // Option<&Entry>
	match tempbib {
	    Some(entry) => {
		condensed_bib.insert(Clone::clone(entry));
	    },
	    None => {
		eprint!("Citekey: {} not found. Ignored.\n", &key);
	    }
	}
    };
    condensed_bib
}

pub fn condense (md_paths: Vec<&str>, bib_path: &str) -> Bibliography {
    let mut text = String::from("");
    for path in md_paths {
	let temp_text = fs::read_to_string(path);
	match temp_text {
	    Ok(content) => {
		text.push_str(&content);
		text.push_str("\n");
	    },
	    Err(_error) => {
		eprint!("Cannot read {}, ignored.\n", path);
	    }
	};
    }
    let keys = extract_citekeys(&text);
    let mut v: Vec<_> = keys.into_iter().collect();
    v.sort();
    let bib = read_bibtex(bib_path);
    extract_bibtex(&v, &bib)
}

pub fn bibcon(md_paths: Vec<&str>, bib_path: &str) {
    let cbib = condense(md_paths, bib_path);
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
	let v: Vec<_> = keys.into_iter().collect();
	let cbib = extract_bibtex(&v, &bib);
	assert_eq!(bib.len(), 6);
	assert_eq!(cbib.len(), 5);
    }
    #[test]
    fn ut_extract_bibtex2() {
	// won't panic when some cite keys are not found
	let text = fs::read_to_string("tests/r1.rmd")
	    .expect("Should have been able to read the file");
	let keys = extract_citekeys(&text);
	let bib = read_bibtex("tests/weat.bib");
	let v: Vec<_> = keys.into_iter().collect();
	let cbib = extract_bibtex(&v, &bib);
	assert_eq!(bib.len(), 6);
	assert_eq!(cbib.len(), 0);
    }
    #[test]
    fn ut_condense() {
	let paths = vec!["tests/r1.rmd", "tests/r2.rmd"];
	let cbib = condense(paths, "tests/main.bib");
	assert_eq!(cbib.len(), 4);
    }
    #[test]
    fn ut_condense2() {
	// with a non-exist file; ignore it
	let paths = vec!["tests/r1.rmd", "tests/r2.rmd", "tests/r3.rmd"];
	let cbib = condense(paths, "tests/main.bib");
	assert_eq!(cbib.len(), 4);
    }
    #[test]
    fn ut_condense3() {
	// return nothing if all files don't exist
	let paths = vec!["tests/r4.rmd", "tests/r3.rmd"];
	let cbib = condense(paths, "tests/main.bib");
	assert_eq!(cbib.len(), 0);	
    }
}
