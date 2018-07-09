// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::cmp::Ordering;
use std::fs;
use std::path::Path;

use utils::{get_options, read_file};

struct Import {
    name: String,
    options: Vec<usize>,
    pos: usize,
}

impl Eq for Import {}
impl PartialEq for Import {
    fn eq(&self, other: &Import) -> bool {
        self.name == other.name
    }
}

impl PartialOrd for Import {
    fn partial_cmp(&self, other: &Import) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Import {
    fn cmp(&self, other: &Import) -> Ordering {
        self.name.cmp(&other.name)
    }
}

fn add_error<P: AsRef<Path>>(p: &P,
                             has_added_filename: &mut bool,
                             errors: &mut Vec<String>,
                             error: String) {
    if !*has_added_filename {
        errors.push(format!("==>> {} <<==", p.as_ref()
                                             .to_str()
                                             .expect("cannot get filename")));
        *has_added_filename = true;
    }
    errors.push(error);
}

fn check_file<P: AsRef<Path>>(p: P, errors: &mut Vec<String>) {
    let content = read_file(&p);
    let lines = content.lines().map(|s| s.trim()).collect::<Vec<_>>();
    let mut imports = Vec::with_capacity(10);
    let mut has_added_filename = false;

    for pos in 0..lines.len() {
        if lines[pos].starts_with("use") {
            let elems = lines[pos].split_whitespace().take(3).collect::<Vec<_>>();
            if elems.len() < 2 {
                continue
            }
            let len = elems[1].len() - 1; // to remove ';'
            imports.push(Import {
                name: elems[1][..len].to_lowercase(),
                options: get_options(pos, &lines),
                pos,
            });
        } else if !lines[pos].starts_with("#") && !lines[pos].starts_with("//") {
            let mut local_errors = 0;
            if imports.is_empty() {
                continue
            }
            for it in 0..imports.len() - 1 {
                if imports[it] > imports[it + 1] {
                    add_error(&p, &mut has_added_filename, errors,
                              format!("ERROR: \"{}\" should be after \"{}\"",
                                      lines[imports[it].pos],
                                      lines[imports[it + 1].pos]));
                    local_errors += 1;
                }
            }
            // TODO: also check inside {}. If line ends with '{' or contains '{', needs to
            // split "," and then trim.
            if local_errors > 0 {
                imports.sort();
                errors.push(
                    format!("\n== Expected output ==\n{}",
                            imports.iter()
                                   .map(|c| format!("{}{}\n",
                                                    c.options
                                                     .iter()
                                                     .map(|o| format!("{}\n", lines[*o]))
                                                     .collect::<String>(),
                                                    lines[c.pos]))
                                   .collect::<String>()));
            }
            imports.clear();
        }
    }
}

#[test]
fn check_imports() {
    let mut errors = Vec::with_capacity(10);

    for entry in fs::read_dir("src").expect("cannot read src") {
        let entry = entry.expect("cannot get entry");
        let path = entry.path();
        if path.is_file() {
            check_file(path, &mut errors);
        }
    }
    println!("{}", errors.join("\n"));
    assert_eq!(errors.len(), 0);
}
