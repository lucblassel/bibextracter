use biblatex::{Bibliography, Entry};
use std::collections::HashSet;
use std::fmt::Write;
use std::fs;

/// Read a bib file
pub fn read(path: &str) -> Bibliography {
    let biblio_f = fs::read_to_string(path).unwrap();
    let bib = Bibliography::parse(&biblio_f).unwrap();

    bib
}

/// Subsets a bib(la)tex bibliography given a set of keys
pub fn subset(keys: &HashSet<String>, source: Bibliography) -> Bibliography {
    let mut output_bib = Bibliography::new();

    for key in keys {
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
pub fn write(bib: Bibliography, filename: String, bibtex: bool, clean: bool, order: Vec<String>) {
    let bib_str: String;

    if !clean && order.len() == 0 {
        if bibtex {
            bib_str = bib.to_bibtex_string();
        } else {
            bib_str = bib.to_biblatex_string();
        }
    } else {
        bib_str = format_biblio(bib, bibtex, order);
    }

    if filename == "stdout" {
        println!("{}", bib_str);
    } else {
        fs::write(filename, bib_str).unwrap();
    }
}

fn format_biblio(bib: Bibliography, bibtex: bool, order: Vec<String>) -> String {
    let mut bib_str = String::new();

    if order.len() == 0 {
        for entry in bib {
            writeln!(bib_str, "{}", format_entry(&entry, bibtex)).unwrap();
        }
    } else {
        for key in order {
            match &bib.get(&key) {
                Some(entry) => writeln!(bib_str, "{}", format_entry(entry, bibtex)).unwrap(),
                None => continue,
            }
        }
    }

    bib_str
}

/// Clean entry formatting with indentation
fn format_entry(entry: &Entry, bibtex: bool) -> String {
    let mut formatted = String::new();

    let entry_str: String;

    if bibtex {
        entry_str = entry.to_bibtex_string().unwrap();
    } else {
        entry_str = entry.to_biblatex_string();
    }

    // let entry_str = entry.to_biblatex_string();
    let lines: Vec<&str> = entry_str.split('\n').collect();

    writeln!(formatted, "{}", lines[0]).unwrap();
    for line in &lines[1..lines.len() - 1] {
        let vals: Vec<&str> = line.split(" = ").collect();
        let spaces = String::from_utf8(vec![b' '; 14 - vals[0].len()]).unwrap();
        writeln!(formatted, "  {}{} = {}", vals[0], spaces, vals[1..].join(" = ")).unwrap();
    }
    writeln!(formatted, "{}", '}').unwrap();

    formatted
}
