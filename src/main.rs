use std::io::{self, BufRead};
use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

const DEBUG: bool = false;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let file: File;
    let mut entries: Vec<&str> = Vec::new();
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
                let parts = ln.trim().split(" ");
                for part in parts {
                    let nvlist: Vec<&str> = part.split("\"").collect();
                    let mut href;
                    let mut modified;
                    for i in 0..nvlist.len() {
                        if nvlist[i].eq("href=") && i + 1 < nvlist.len() {
//                            println!("HREF {}", nvlist[i + 1]);
                            href = nvlist[i + 1];
                        } else if nvlist[i].eq("modified=") && i + 1 < nvlist.len() {
//                            println!("MOD {}", nvlist[i + 1]);
                            modified = nvlist[i + 1];
                        }
                    }
                }
            }
        }
    }
}
