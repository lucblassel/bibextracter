# bibextracter

## Description

Extract cited references from a `.tex` source file and use them to subset a large reference `.bib` file. 
Inspired by `bibtool` but only uses the tex source and not the `.aux` file.  
*(This is also just a little project to try Rust out)*

## Usage

```
Usage: bibextracter [OPTIONS] --bib <BIB> <TEX>...

Arguments:
  <TEX>...  The .tex source file(s) containing citations

Options:
  -b, --bib <BIB>  The .bib bibliography file(s) to extract entries from
  -o, --out <OUT>  The path to the output .bib file [default: stdout]
  -B, --bibtex     Format the output as bibtex instead of biblatex
  -h, --help       Print help information
  -V, --version    Print version information
```

## Installation
Make sure you have rust installed and use `cargo build --release` to compile binary. 