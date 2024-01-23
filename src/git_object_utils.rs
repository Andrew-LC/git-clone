use flate2::{self, read::ZlibDecoder, write::ZlibEncoder, Compression};
use std::io::{Read, Write};
use std::path::Path;
use anyhow;
use std::fs::{self, File};
use ring::digest::SHA256;

// Types of git objects
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum GitObjectType {
    Blob,
    Tree,
    Commit,
}

// Git object
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Object {
    pub kind: GitObjectType,
    pub size: usize,
    pub data: String,
}

impl Object {
    // Read object from file
    pub fn from_sha(path: &str) -> anyhow::Result<Self> {
	let git_object_path = ".git/objects/";
	let (path, file) = path.split_at(2); // Consider extracting two characters at a time
	let file_location = Path::new(&git_object_path).join(path).join(file);

	if let Ok(file) = File::open(&file_location) {
	    let mut decoder = ZlibDecoder::new(file);
	    let mut decompressed = String::new();
	    if let Ok(_) = decoder.read_to_string(&mut decompressed) {
		let (header, data) = decompressed
		    .split_once('\0')
		    .unwrap();

		let (_object_type, size) = header
		    .split_once(' ')
		    .unwrap();

		Ok(
		    Object {
			kind: GitObjectType::Blob,
			size: size.parse::<usize>().unwrap(),
			data: data.to_string(),
		    }
		)
	    } else {
		eprintln!("Error reading file");
		Err(anyhow::Error::msg("Error reading file"))
	    }
	} else {
	    eprintln!("File not found");
	    Err(anyhow::Error::msg("File not found"))
	}
    }

    // Hash object
    pub fn hash(file_content: &str) -> anyhow::Result<String> {
	let sha1 = ring::digest::digest(&SHA256, file_content.as_bytes());  
	let hex_hash = sha1.as_ref().iter().map(|b| format!("{:02x}", b)).collect::<String>();
	let (hex_hash, _) = hex_hash.split_at(40);

	Ok(hex_hash.to_string())
    }

    // Write object to file
    pub fn hash_object(path: &str) -> anyhow::Result<(String, String)> {
	let file_content = fs::read_to_string(path)?;
	let sha = Object::hash(&file_content)?;

	let git_object_path = ".git/objects/";
	let (path, file) = sha.split_at(2);
	let dir_location = Path::new(&git_object_path).join(path);

	// Create the necessary directories if they don't exist
	fs::create_dir_all(&dir_location)?;

	let file_location = dir_location.join(file);

	if let Ok(file) = File::create(&file_location) {
            let mut encoder = ZlibEncoder::new(file, Compression::default());
	    let compressed_data = format!("blob {}\0{}", file_content.len(), file_content);
            encoder.write_all(&compressed_data.as_bytes())?;
            Ok((sha, file_location.to_str().unwrap().to_string()))
	} else {
            eprintln!("Error creating file at location: {:?}", file_location);
            eprintln!("Error details: {:?}", std::io::Error::last_os_error());
            Err(anyhow::Error::msg("Error creating file"))
	}
    }
}
