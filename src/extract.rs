use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
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
fn keys(filename: &str) -> Result<HashSet<String>, std::io::Error> {
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

pub fn all_keys(filenames: Vec<String>) -> Result<HashSet<String>, std::io::Error> {
    let mut all_keys: HashSet<String> = HashSet::new();

    for filename in filenames {
        let filename_keys = keys(&filename).unwrap();
        all_keys.extend(filename_keys);
    }

    Ok(all_keys)
}
