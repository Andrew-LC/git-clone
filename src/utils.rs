use flate2::{self, read::ZlibDecoder};
use std::io::Read;
use std::fs::File;
use std::path::Path;

pub fn decode(path: &str) {
    let git_object_path = ".git/objects/";
    let (path, file) = path.split_at(2);
    let file_location = Path::new(&git_object_path).join(path).join(file);

    if file_location.exists() {
	if let Ok(file) = File::open(file_location) {
	    let mut decoder = ZlibDecoder::new(file);
	    let mut s = String::new();
	    if let Ok(_) = decoder.read_to_string(&mut s) {
		print!("{}", s);
	    } else {
		eprintln!("Zlib Error!");
	    }    
	}
    };
}
