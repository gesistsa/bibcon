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
