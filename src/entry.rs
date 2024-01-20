use std::fs;
use std::path::Path;

#[allow(dead_code)]
pub struct Entry {
    pub path: String,
    pub sha: String,
    pub file_type: String,
}

impl Entry {
    pub fn new(path: String, sha: String, file_type: String) -> Self {
	Self { path, sha, file_type }
    }

    pub fn parse(sha: &str) -> Vec<Entry> {
	let (dir, file) = sha.split_at(2);
	let object_path = Path::new("./git/objects").join(dir).join(file);
	let mut entries = vec![];

	if let Ok(data) = fs::read_to_string(&object_path) {
            for line in data.lines() {
		let (first, filename) = line.split_once(' ').unwrap();
		let collected_data: Vec<&str> = first.split_whitespace().collect();
		let (_, file_type_sha) = collected_data.split_at(2);
		let (file_type, sha) = file_type_sha.split_at(1);

		entries.push(Entry::new(filename.to_string(), sha.join(" ").to_string(), file_type.join(" ").to_string()));
            }
	}

	entries
    }
}
