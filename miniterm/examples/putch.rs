#![allow(unused)]

use std::io::{stdin, stdout, Write};

use miniterm::IntoRawMode;

fn main() {
    let mut cin = stdin();
    let mut cout = stdout();
    let mut term = cout.into_raw_mode().unwrap();
    term.write_all(b"smh\n");
}
