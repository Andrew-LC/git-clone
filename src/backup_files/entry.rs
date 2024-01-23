use std::fs::{self, File, DirEntry};
use std::path::Path;
use std::io::Read;

#[allow(dead_code)]
pub struct Entry {
    pub filename: String,
    pub sha: String,
    pub file_type: String,
}

impl Entry {
    pub fn _new(filename: String, sha: String, file_type: String) -> Self {
	Self { filename, sha, file_type }
    }

    pub fn _parse(path: &str) -> Vec<Entry> {
	let (dir, file) = path.split_at(2);
	let object_path = Path::new("./git/objects").join(dir).join(file);
	let mut file_open = File::open(object_path).unwrap();
	let mut contents = String::new();
	let entries = vec![];

	let _ = file_open.read_to_string(&mut contents);
	print!("{}", contents);

	entries
    }

    pub fn ls_tree() {
	let mut dir_contents = fs::read_dir(".").unwrap()
            .into_iter()
            .filter(|entry| entry.as_ref().unwrap().path().file_name().unwrap() != ".git")
            .collect::<Result<Vec<DirEntry>, std::io::Error>>().unwrap();

	dir_contents.sort_by(|a, b| a.path().partial_cmp(&b.path()).unwrap());

	for item in dir_contents.iter() {
            println!("{}", item.path().file_name().unwrap().to_str().unwrap());

	}
    }
}
