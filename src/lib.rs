use lazy_static::lazy_static;

use regex::Regex;
use std::collections::HashSet;

pub fn extract_citekeys(text: &str) -> HashSet<&str>{
    lazy_static! {
        static ref HASHTAG_REGEX : Regex = Regex::new(
                r"(?P<ini>^|[\[,; -])@(?P<key>[[[:alnum:]]_\-:]+)"
            ).unwrap();
    }
    HASHTAG_REGEX.captures_iter(text).map(|c| c.name("key").unwrap().as_str()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn extract_citekeys_basic() {
	let text = "[@abc;@efg p. 1234] chung-hong.chan@gesis.org @hij \\@ref(fig:fig5)";
	let keys = extract_citekeys(&text);
	assert_eq!(keys.len(), 3);
	assert_eq!(keys.contains("chan"), false);
	assert_eq!(keys.contains("abc"), true);
	assert_eq!(keys.contains("efg"), true);
	assert_eq!(keys.contains("hij"), true);
    }
    #[test]
    fn extract_citekeys_real1() {
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
}
