use biblatex::Bibliography;
use std::collections::HashSet;
use std::fs;

/// Read a bib file
pub fn read(path: &str) -> Bibliography {
    let biblio_f = fs::read_to_string(path).unwrap();
    let bib = Bibliography::parse(&biblio_f).unwrap();

    bib
}

/// Subsets a bib(la)tex bibliography given a set of keys
pub fn subset(keys: HashSet<String>, source: Bibliography) -> Bibliography {
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

/// Write bibliography to stdout or a file
pub fn write(bib: Bibliography, filename: String, bibtex: bool) {
    let bib_str: String;

    if bibtex {
        bib_str = bib.to_bibtex_string();
    } else {
        bib_str = bib.to_biblatex_string();
    }

    if filename == "stdout" {
        println!("{}", bib_str);
    } else {
        fs::write(filename, bib_str).unwrap();
    }
}
