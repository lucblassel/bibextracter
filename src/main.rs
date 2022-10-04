use clap::Parser;
use std::collections::HashSet;
use std::fs;

pub mod biblio;
pub mod extract;

#[derive(Parser, Debug)]
#[command(author = "Luc Blassel", version)]
#[command(about="A tool to extract reference entries, \
cited in one or more tex files, \
from a bib files.", long_about=None)]
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
    #[arg(short = 'B', long)]
    bibtex: bool,
}

fn main() {
    let args = Cli::parse();

    let mut keys: HashSet<String> = HashSet::new();

    for tex in args.tex {
        let tex_keys = extract::get_keys(&tex).unwrap();
        keys.extend(tex_keys);
    }

    let bib = biblio::read_biblio(&args.bib);
    let output_bib = biblio::subset_biblio(keys, bib);

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
