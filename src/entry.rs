use flate2::{self, read::ZlibDecoder};
use std::io::Read;
use std::fs::File;
use std::path::Path;

#[allow(dead_code)]
pub struct Entry<'a> {
    path: &'a str,
    sha: &'a str
}

// Example of how the tree format is: tree [content size]\0[Entries having references to other trees and blobs]

pub fn parse(tree: &str) {
    let git_object_path = ".git/";
    let file_location = Path::new(&git_object_path).join(tree);

    if let Ok(file) = File::open(&file_location) {
	let mut decoder = ZlibDecoder::new(file);
	let mut decompressed = String::new();
	let _ =decoder.read_to_string(&mut decompressed);
	println!("{:?}", decompressed);
    }   
}
