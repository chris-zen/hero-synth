pub mod sin;
pub mod saw;

use std::fs::File;
use std::path::PathBuf;
use std::error::Error;
use std::io;
use std::io::prelude::*;

pub const DEFAULT_WAVETABLE_SIZE: usize = 1 << 14;

static HEADER: &'static str = "//!
//! Lookup Table for a {} waveform
//!
";

static TABLE_START: &'static str = "
pub const LUT: &'static [f64] = &[
";

static TABLE_END: &'static str = "];\n";


pub fn gen_lut(base_path: &str, name: &str, generator: &mut Iterator<Item=f64>) -> io::Result<()> {
    let mut file_name = String::from(name) + ".rs";
    let path = PathBuf::from(base_path).join(file_name);

    println!("Generating {} ...", path.display());

    let mut out_file = try!(File::create(&path));
    try!(out_file.write(HEADER.replace("{}", name).as_bytes()));
    try!(out_file.write(TABLE_START.as_bytes()));

    for value in generator {
        out_file.write(format!("    {:.50}f64,\n", value).as_bytes()).unwrap();
    }

    try!(out_file.write(TABLE_END.as_bytes()));

    Ok(())
}
