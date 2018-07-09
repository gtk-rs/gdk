// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::cmp::Ordering;

use utils::read_file;

struct Crate<'a> {
    name: &'a str,
    options: Vec<usize>,
    pos: usize,
}

impl<'a> Eq for Crate<'a> {}
impl<'a> PartialEq for Crate<'a> {
    fn eq(&self, other: &Crate) -> bool {
        self.name == other.name
    }
}

impl<'a> PartialOrd for Crate<'a> {
    fn partial_cmp(&self, other: &Crate) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Crate<'a> {
    fn cmp(&self, other: &Crate) -> Ordering {
        self.name.cmp(&other.name)
    }
}

fn get_options(mut pos: usize, lines: &[&str]) -> Vec<usize> {
    let mut positions = Vec::with_capacity(1);

    while pos > 0 && lines[pos - 1].starts_with("#[") {
        pos -= 1;
        positions.push(pos);
    }
    positions
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
            let len = elems[2].len() - 1;
            crates.push(Crate {
                name: &elems[2][..len],
                options: get_options(pos, &lines),
                pos,
            });
        }
    }

    let mut errors = 0;
    for pos in 0..crates.len() - 1 {
        if crates[pos] > crates[pos + 1] {
            println!("ERROR: \"{}\" should be after \"{}\"",
                     lines[crates[pos].pos],
                     lines[crates[pos + 1].pos]);
            errors += 1;
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
