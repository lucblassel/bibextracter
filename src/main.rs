use biblatex::Bibliography;
use clap::Parser;
use regex::Regex;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

/// Possible citing commands in tex document
static COMMANDS: [&str; 8] = [
    "cite",
    "parencite",
    "footcite",
    "footcitenext",
    "textcite",
    "smartcite",
    "supercite",
    "autocite",
];

/// Extracts cited keys from a tex document
fn get_keys(filename: &str) -> Result<HashSet<String>, std::io::Error> {
    let joined = COMMANDS.join("|");
    let mut re_s = String::from(r"\\(");
    re_s.push_str(&joined);
    re_s.push_str(r")\*?(\[.*\])?\{([\w,-]+)\}");
    let re = Regex::new(&re_s).unwrap();

    let mut keys = HashSet::new();

    let f = BufReader::new(File::open(filename).unwrap());
    for line in f.lines() {
        match line {
            Ok(line) => {
                for cap in re.captures_iter(&line) {
                    let references: &str = &cap[3];
                    for reference in references.split(",") {
                        keys.insert(String::from(reference));
                    }
                }
            }
            Err(err) => return Err(err),
        };
    }

    Ok(keys)
}

/// Subsets a bib(la)tex bibliography given a set of keys
fn subset_biblio(keys: HashSet<String>, source: Bibliography) -> Bibliography {
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

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about="A tool to extract reference entries, cited in one or more tex files, from one or more bib files.", long_about=None)]
struct Cli {
    /// The .tex source file(s) containing citations
    #[arg(required = true)]
    tex: Vec<String>,

    /// The .bib bibliography file(s) to extract entries from
    #[arg(short, long, required = true)]
    bib: String,

    /// The path to the output .bib file
    #[arg(short, long, default_value_t=String::from("stdout"))]
    out: String,

    /// Format the output as bibtex instead of biblatex
    #[arg(long)]
    bibtex: bool,
}

fn main() {
    let args = Cli::parse();
    eprintln!("Args: {:?}", args);

    let mut keys: HashSet<String> = HashSet::new();

    for tex in args.tex {
        let tex_keys = get_keys(&tex).unwrap();
        keys.extend(tex_keys);
    }

    let biblio_f = fs::read_to_string(args.bib).unwrap();
    let bib = Bibliography::parse(&biblio_f).unwrap();

    let output_bib = subset_biblio(keys, bib);

    let bib_str: String;
    if args.bibtex {
        bib_str = output_bib.to_bibtex_string();
    } else {
        bib_str = output_bib.to_biblatex_string();
    }

    if args.out == "stdout" {
        println!("{}", bib_str)
    } else {
        fs::write(args.out, bib_str).unwrap();
    }
}
