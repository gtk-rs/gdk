// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(p: P) -> String {
    let mut f = File::open(p).expect("read_file::open failed");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("read_file::read_to_end failed");
    content
}

pub fn get_options(mut pos: usize, lines: &[&str]) -> Vec<usize> {
    let mut positions = Vec::with_capacity(1);

    while pos > 0 && lines[pos - 1].starts_with("#[") {
        pos -= 1;
        positions.push(pos);
    }
    positions
}
