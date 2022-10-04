use biblatex::Bibliography;
use std::collections::HashSet;
use std::fs;

pub fn read_biblio(path: &str) -> Bibliography {
    let biblio_f = fs::read_to_string(path).unwrap();
    let bib = Bibliography::parse(&biblio_f).unwrap();

    bib
}

/// Subsets a bib(la)tex bibliography given a set of keys
pub fn subset_biblio(keys: HashSet<String>, source: Bibliography) -> Bibliography {
    let mut output_bib = Bibliography::new();

    for key in &keys {
        match source.get(key) {
            Some(entry) => {
                output_bib.insert(entry.clone());
            }
            None => eprintln!("Key {} not found in bibliography file.", key),
        }
    }

    output_bib
}
