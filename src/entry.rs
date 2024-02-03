use std::path::{Path, PathBuf};
use anyhow::Result;
use std::fs;
use flate2::{self, read::ZlibDecoder, write::ZlibEncoder, Compression};
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use super::git_object_utils::Object;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Entry {
    pub mode: String,
    pub filename: String,
    pub sha: Vec<u8>,
}

pub fn ls_tree(tree: &str, pretty_print: bool) {
    let entries = parse(&tree).unwrap();

    if !pretty_print {
	for name in entries {
	    println!("{}", name.filename);
	}
    } else {
	for name in entries {
	    println!("{}  {}   {:?}", name.mode, name.filename, std::str::from_utf8(&name.sha));
	}
    }
}

pub fn parse(tree: &str) -> Result<Vec<Entry>> {
    let git_object_path = ".git/objects/";
    let (dir, file) = tree.split_at(2);
    let file_location = Path::new(&git_object_path).join(dir).join(file);
    let mut file = fs::File::open(file_location)?;
    let mut decompressed: Vec<u8> = Vec::new();

    let mut decoder = ZlibDecoder::new(&mut file);
    decoder.read_to_end(&mut decompressed)?;

    let first_sep = decompressed.iter().position(|x| x == &b'\0').unwrap();
    let mut rest = &decompressed[first_sep + 1..];
    let mut children = Vec::new();

    while !rest.is_empty() {
        let sep = rest.iter().position(|x| x == &b' ').unwrap();
        let mode = std::str::from_utf8(&rest[..sep]).unwrap().to_string();
        rest = &rest[sep + 1..];
        let sep = rest.iter().position(|x| x == &b'\0').unwrap();
        let filename = std::str::from_utf8(&rest[..sep]).unwrap().to_string();
        let sha: Vec<u8> = rest[sep + 1..sep + 21].try_into().unwrap();
	let sha = sha.to_vec();
        rest = &rest[sep + 21..];
        children.push(Entry {
            mode,
            filename,
            sha,
        });
    }

    Ok(children)
}

pub fn write_tree() -> Result<()> {
    let mut entries: Vec<Entry> = Vec::new();
    let tree_hash = visit_dir(Path::new("."), &mut entries)?;

    println!("{}", String::from_utf8(tree_hash)?);

    Ok(())
}

fn visit_dir(dir: &Path, entries: &mut Vec<Entry>) -> Result<Vec<u8>> {
    assert!(dir.is_dir());

    for entry in fs::read_dir(dir)? {
	let entry = entry?;
	let path = entry.path();
	let filename = path.file_name().unwrap().to_str().unwrap().to_string();

	if filename == ".git" {
	    continue;
	}

	let tree_entry = match path.is_dir() {
	    true => {
		let tree_dir = visit_dir(&path, &mut Vec::new())?;
		build_tree_entry_from_dir(path, tree_dir)}
	    false => {
		build_tree_entry_file(&path)
	    }
	};
	entries.push(tree_entry);
    }
    entries.sort_by(|a, b| a.filename.cmp(&b.filename));
    let tree_file = create_tree_file(entries).unwrap();
    let tree_hash = save_to_disk(&tree_file)?;
    Ok(tree_hash)
}

fn build_tree_entry_file(path: &Path) -> Entry {
    let filename = path.file_name().unwrap().to_str().unwrap().to_string();
    let mode = get_tree_mode_by_path(path.to_path_buf());
    let (path_hash, _) = Object::hash_object(&path).unwrap();
    let path_hash = path_hash.into_bytes();
    Entry {
        mode,
        filename,
        sha: path_hash,
    }
}

fn build_tree_entry_from_dir(path: PathBuf, dir_hash: Vec<u8>) -> Entry {
    let filename = path.file_name().unwrap().to_str().unwrap().to_string();
    let mode = get_tree_mode_by_path(path);
    let sha = dir_hash;

    Entry {
        mode,
        filename,
        sha
    }
}

fn get_tree_mode_by_path(path: PathBuf) -> String {
    match path.metadata() {
        Ok(metadata) => {
            let is_exec = metadata.permissions().mode() & 0o111 != 0;
            if is_exec {
                "100755".to_string()
            } else if metadata.is_file() {
                "100644".to_string()
            } else if metadata.is_dir() {
                "40000".to_string()
            } else if metadata.is_symlink() {
                "120000".to_string()
            } else {
                panic!("Unknown file type");
            }
        }
        Err(_) => panic!("Error reading file"),
    }
}


fn tree_as_bytes(tree_entry: &Entry) -> Vec<u8> {
    let mut tree_bytes = Vec::new();
    let mode = format!("{} ", &tree_entry.mode);
    let filename = format!("{}\0", &tree_entry.filename);
    let sha = &tree_entry.sha;

    tree_bytes.extend(mode.as_bytes());
    tree_bytes.extend(filename.as_bytes());
    tree_bytes.extend(sha);

    tree_bytes
}

fn create_tree_file(entries: &mut Vec<Entry>) -> Result<Vec<u8>> {
    let mut index = Vec::new();
    let objects = entries
	.iter()
        .map(|entry: &Entry| tree_as_bytes(&entry))
        .collect::<Vec<Vec<u8>>>();
    let size = objects.iter().fold(0, |acc, object| acc + object.len());

    index.extend(format!("tree {size}\0").as_bytes());

    for object in objects {
        index.extend(object)
    }

    Ok(index)
}

fn save_to_disk(content: &[u8]) -> Result<Vec<u8>> {
    let sha = Object::hash(std::str::from_utf8(&content)
			   .map_err(|e| anyhow::Error::msg(format!("Error converting to UTF-8: {}", e)))?)?;

    let mut encoded = Vec::new();
    let mut encoder = ZlibEncoder::new(&mut encoded, Compression::default());
    encoder.write_all(content)?;

    let compressed = encoder.finish()?.to_owned();

    let git_object_path = ".git/objects/";
    let (path, file) = sha.split_at(2);
    let dir_location = Path::new(&git_object_path).join(path);

    println!("{:?}", dir_location);

    // Create the necessary directories if they don't exist
    fs::create_dir_all(&dir_location)?;

    if dir_location.exists() {
	return Ok(sha.into());
    }

    let file_location = dir_location.join(file);

    std::fs::write(file_location, &compressed)?;
    
    Ok(sha.into_bytes())
}


// fn hash_object(data: &Vec<u8>, write: bool) -> Result<Vec<u8>> {
//     let mut tree_file = vec![];
//     tree_file.extend(format!("tree {}", data.len()).as_bytes());
//     tree_file.push(b'\x00');

//     tree_file.extend_from_slice(&data);

//     let sha = Object::hash(std::str::from_utf8(&tree_file)
// 			   .map_err(|e| anyhow::Error::msg(format!("Error converting to UTF-8: {}", e)))?)?;

//     if write {
// 	let git_object_path = ".git/objects/";
// 	let (path, file) = sha.split_at(2);
// 	let dir_location = Path::new(&git_object_path).join(path);

// 	// Create the necessary directories if they don't exist
// 	fs::create_dir_all(&dir_location)?;

// 	let file_location = dir_location.join(file);

// 	if let Ok(file) = fs::File::create(&file_location) {
//             let mut encoder = ZlibEncoder::new(file, Compression::default());
// 	    let character = String::from_utf8(tree_file.to_vec());
// 	    if let Ok(_) = encoder.finish() {
// 		Ok(sha.into())
// 	    } else {
// 		Err(anyhow::Error::msg("Error compressing data"))
// 	    }
// 	} else {
//             eprintln!("Error creating file at location: {:?}", file_location);
//             eprintln!("Error details: {:?}", std::io::Error::last_os_error());
//             Err(anyhow::Error::msg("Error creating file"))
// 	}
//     } else {
// 	eprintln!("Error details: {:?}", std::io::Error::last_os_error());
// 	Err(anyhow::Error::msg("Error creating file"))
//     }
// }
