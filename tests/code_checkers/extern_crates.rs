// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::cmp::Ordering;

use utils::{get_options, read_file};

struct Crate {
    name: String,
    options: Vec<usize>,
    pos: usize,
}

impl Eq for Crate {}
impl PartialEq for Crate {
    fn eq(&self, other: &Crate) -> bool {
        self.name == other.name
    }
}

impl PartialOrd for Crate {
    fn partial_cmp(&self, other: &Crate) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Crate {
    fn cmp(&self, other: &Crate) -> Ordering {
        self.name.cmp(&other.name)
    }
}

#[test]
fn check_extern_crates() {
    let content = read_file("src/lib.rs");
    let lines = content.lines().map(|s| s.trim()).collect::<Vec<_>>();
    let mut crates = Vec::with_capacity(10);

    for pos in 0..lines.len() {
        if lines[pos].starts_with("extern") {
            let elems = lines[pos].split_whitespace().take(3).collect::<Vec<_>>();
            if elems.len() < 3 || elems.get(1) != Some(&"crate") {
                continue
            }
            let len = elems[2].len() - 1; // to remove ';'
            crates.push(Crate {
                name: elems[2][..len].to_lowercase(),
                options: get_options(pos, &lines),
                pos,
            });
        }
    }

    let mut errors = 0;
    if !crates.is_empty() {
        for pos in 0..crates.len() - 1 {
            if crates[pos] > crates[pos + 1] {
                println!("ERROR: \"{}\" should be after \"{}\"",
                         lines[crates[pos].pos],
                         lines[crates[pos + 1].pos]);
                errors += 1;
            }
        }
    }
    if errors > 0 {
        crates.sort();
        println!("\n== Expected output ==\n{}",
                 crates.iter()
                       .map(|c| format!("{}{}\n",
                                        c.options
                                         .iter()
                                         .map(|o| format!("{}\n", lines[*o]))
                                         .collect::<String>(),
                                        lines[c.pos]))
                       .collect::<String>());
    }
    assert_eq!(errors, 0);
}
