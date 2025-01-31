use percent_encoding::percent_decode_str;
use std::io::{self, BufRead};
use std::env;
use std::fs::File;
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn url_decode(encoded: &str) -> String {
    percent_decode_str(encoded).decode_utf8_lossy().to_string()
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut entries: Vec<String> = Vec::new();
    if let Some(path) = env::home_dir() {
        let mut filename = format!("{}/{}", path.display(), ".local/share/recently-used.xbel");
        if args.len() == 2 {
            filename = args[1].clone();
        }

        if let Ok(lines) = read_lines(filename) {
            for line in lines.flatten() {
                let ln = line.trim();
                if !ln.starts_with("<bookmark href=") {
                    continue
                }
                let mut href = String::new();
                let mut modified = String::new();
                let parts = ln.trim().split(" ");
                for part in parts {
                    let nvlist: Vec<&str> = part.split("\"").collect();
                    let mut i = 0;
                    while i < nvlist.len() {
                        if nvlist[i].eq("href=") && i + 1 < nvlist.len() {
                            href = nvlist[i + 1].to_string();
                            if href.starts_with("file://") {
                                href = href[7..].to_string();
                            } else {
                                href = String::new();
                            }
                            if href.contains("%") {
                                href = url_decode(&href);
                            }
                            i = i + 2;
                        } else if nvlist[i].eq("modified=") && i + 1 < nvlist.len() {
                            modified = nvlist[i + 1].to_string();
                            i = i + 2;
                        } else {
                            i = i + 1;
                        }
                    }
                }
                if modified.len() > 0 && href.len() > 0 {
                    let comp = format!("{} {}", modified, href);
                    entries.push(comp);
                }
            }
        }
    }

    entries.sort();
    entries.reverse();

    for e in entries {
        if e.len() > 28 {
            // print filename only
            println!("{}", &e[28..]);
        }
    }
}
