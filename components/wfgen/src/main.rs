extern crate docopt;
extern crate wfgen;

use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::error::Error;
use std::io;
use std::io::prelude::*;
use docopt::Docopt;


fn main() {
    let args = Docopt::new(USAGE)
                        .and_then(|dopt| dopt.parse())
                        .unwrap_or_else(|e| e.exit());

    let base_path = args.get_str("<output>");

    wfgen::gen_lut(base_path, "sin",
                   &mut wfgen::sin::SinGen::new(wfgen::DEFAULT_WAVETABLE_SIZE)).unwrap();
}
