use clap::Parser;

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

    let keys = extract::all_keys(args.tex).unwrap();

    let bib = biblio::read(&args.bib);
    let output_bib = biblio::subset(keys, bib);

    biblio::write(output_bib, args.out, args.bibtex);
}
