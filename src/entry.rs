#[allow(dead_code)]
#[allow(unused_imports)]
use std::path::Path;
use std::fs::File;
use flate2::{self, read::ZlibDecoder};
use std::io::Read;

use anyhow::Result;

pub struct Entry {
    pub mode: String,
    pub file_type: String,
    pub sha: String,
    pub filename: String,
}

impl Entry {
    pub fn _new(mode: String, sha: String, file_type: String, filename: String) -> Self {
	Self { mode, file_type, sha, filename  }
    }

    pub fn parse(path: &str) -> Result<Vec<Entry>, ()> {
	let (path, file) = path.split_at(2);
	let object_path = Path::new("./git/objects/").join(path).join(file);
	let entries = vec![];

	if let Ok(file) = File::open(&object_path) {
	    let mut decoder = ZlibDecoder::new(file);
	    let mut s = String::new();
	    if let Ok(_) = decoder.read_to_string(&mut s) {
		print!("{}", s);
	    } else {
		panic!("Zlib Error!");
	    }    
	}

	Ok(entries)
    }
}
