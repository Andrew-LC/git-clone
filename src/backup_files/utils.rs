use flate2::{self, read::ZlibDecoder, write::ZlibEncoder, Compression};
use std::io::Write;
#[allow(unused_imports)]
use std::fs::{self, File, DirEntry};
use std::path::{Path, PathBuf};
use ring::digest::{Digest, SHA256};
use std::io::Read;
use anyhow::Result;
use std::collections::BTreeMap;
use std::os::unix::fs::PermissionsExt;

use crate::tree::Tree;

pub fn create_object_path(git_path: &str, sha1: &Digest) -> anyhow::Result<PathBuf> {
    let hex_hash = sha1.as_ref().iter().map(|b| format!("{:02x}", b)).collect::<String>();
    // get the firsts 40 chars of hex_hash
    let (hex_hash, _) = hex_hash.split_at(40);
    println!("hex_hash: {}", hex_hash);
    
    let (dir, file) = hex_hash.split_at(2);

    let object_path = Path::new(git_path)
        .join(dir);

    // Create directories if they don't exist
    fs::create_dir_all(&object_path)?;

    Ok(object_path.join(file))
}


pub fn write_object(object_path: &Path, content: String) -> anyhow::Result<()> {
    let file = File::create(object_path)?;
    let mut encoder = ZlibEncoder::new(file, Compression::default());

    if let Ok(_) = encoder.write_all(content.as_bytes()) {
	encoder.finish()?;  
	Ok(())
    } else {
	panic!("Failed to compress!");
    }  
}

pub fn decompress(path: &str) {
    let git_object_path = ".git/objects/";
    let (path, file) = path.split_at(2); // Consider extracting two characters at a time
    let file_location = Path::new(&git_object_path).join(path).join(file);

    if let Ok(file) = File::open(&file_location) {
        let metadata = file.metadata();
        match metadata {
            Ok(metadata) => {
                if metadata.is_file() {
                    let mut decoder = ZlibDecoder::new(file);
                    let mut s = String::new();
                    if let Ok(_) = decoder.read_to_string(&mut s) {
                        print!("{}", s);
                        return;
                    }
                }
            }
            Err(e) => eprintln!("Error reading file metadata: {}", e),
        }
    }

    eprintln!("Could not decompress the file!");
}

pub fn hash(file_content: &str) -> Digest {
    let sha1 = ring::digest::digest(&SHA256, file_content.as_bytes());  

    sha1
}

pub fn read_dir() -> Result<String> {
    let git_object_path = ".git/objects/";
    let mut tree_map = BTreeMap::new();
    let current_dir = Path::new(".");

    visit_dir(current_dir, &mut tree_map)?;

    let mut tree_content = String::new();
    for (_, entry) in &tree_map {
        tree_content.push_str(entry);
    }

    let tree_hash = hash(&tree_content);
    let hex_hash = tree_hash.as_ref().iter().map(|b| format!("{:02x}", b)).collect::<String>();
    // get the firsts 40 chars of hex_hash
    let (hex_hash, _) = hex_hash.split_at(40);
    let tree_filename = create_object_path(git_object_path, &tree_hash)?;

    // Write the tree object to a file
    write_object(&tree_filename, tree_content)?;

    // Print the 40-char SHA
    Ok(hex_hash.to_string())
}

fn visit_dir(dir: &Path, tree_map: &mut BTreeMap<String, String>) -> Result<()> {
    let entries = fs::read_dir(dir)?.filter_map(|entry| entry.ok());

    for entry in entries {
        let path = entry.path();
        let file_name = entry.file_name().to_string_lossy().to_string();
        let metadata = entry.metadata()?;
        let permissions = metadata.permissions();
        let is_executable = permissions.mode() & 0o111 != 0;

        if path.is_dir() {
            visit_dir(&path, tree_map)?;
        } else {
            let blob_content = fs::read_to_string(&path)?;
            let blob_hash = hash(&blob_content);
	    let blob_hash = blob_hash.as_ref().iter().map(|b| format!("{:02x}", b)).collect::<String>();
            let hex_hash = blob_hash
                .as_bytes()
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();
            let (hex_hash, _) = hex_hash.split_at(40);

            tree_map.insert(
                file_name.clone(),
                Tree::new(
                    if is_executable { "100755".to_string() } else { "100644".to_string() },
                    file_name,
                    hex_hash.to_string(),
                ).to_tree_object_string(),
            );
        }
    }

    Ok(())
}
