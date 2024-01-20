#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use flate2::{self, read::ZlibDecoder, write::ZlibEncoder, Compression};
use std::io::Read;
use std::io::Write;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use ring::digest::{Digest, SHA256};

#[allow(dead_code)]
enum GitObj {
    Blob,
    Tree,
    Commit
}

mod entry;
use entry::Entry;

fn create_object_path(git_path: &str, sha1: &Digest) -> anyhow::Result<PathBuf> {
    let hex_hash = sha1.as_ref().iter().map(|b| format!("{:02x}", b)).collect::<String>();
    let (dir, file) = hex_hash.split_at(2);

    let object_path = Path::new(git_path)
        .join(dir);

    // Create directories if they don't exist
    fs::create_dir_all(&object_path)?;

    Ok(object_path.join(file))
}


fn write_object(object_path: &Path, content: String) -> anyhow::Result<()> {
    let file = File::create(object_path)?;
    let mut encoder = ZlibEncoder::new(file, Compression::default());

    if let Ok(_) = encoder.write_all(content.as_bytes()) {
	encoder.finish()?;  
	Ok(())
    } else {
	panic!("Failed to compress!");
    }  
}

fn hash(filename: &str) -> Digest {
    let sha1 = ring::digest::digest(&SHA256, filename.as_bytes());  

    sha1
}


fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
	"init" => {
	    let path = env::current_dir().unwrap();
	    let git_path = path.join(".git");
	    if git_path.exists() {
		println!("Reinitialized existing Git repository in {}", git_path.display());
	    } else {
		fs::create_dir(".git").unwrap();
		fs::create_dir(".git/objects").unwrap();
		fs::create_dir(".git/refs").unwrap();
		fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
		println!("Initialized git directory")
	    }   
	},
	"cat-file" => {
	    match args[2].as_str() {
		"-p" => {
		    let git_object_path = ".git/objects/";
		    let (path, file) = &args[3].split_at(2);
		    let file_location = Path::new(&git_object_path).join(path).join(file);

		    if file_location.exists() {
			if let Ok(file) = File::open(file_location) {
			    let mut decoder = ZlibDecoder::new(file);
			    let mut s = String::new();
			    if let Ok(_) = decoder.read_to_string(&mut s) {
				print!("{}", s);
			    } else {
				panic!("Zlib Error!");
			    }    
			}
                    };
		},
		_ => todo!()
	    }
	},
	"hash-object" => {
	    let command = &args[2];
	    
	    match command.as_str() {
		"-w" => {
		    let filename = &args[3];
		    let git_object_path = ".git/objects/";
		    let sha1 = hash(&filename);
		    let file_content = fs::read_to_string(filename)?;

		    // Create the object path based on the SHA1 hash
		    let object_path = create_object_path(git_object_path, &sha1)?;

		    // Compress and write the file content to the object path
		    write_object(&object_path, file_content)?;

		    println!("Object successfully written to: {:?}", object_path);
		},
		_ => todo!()
	    }
	},
	"ls-tree" => {
	    let command = &args[3];
	    match command.as_str() {
		"--name-only" => {
		    let tree_hash = &args[4]; 
		    let entries: Vec<Entry> = Entry::parse(tree_hash);

		    for entry in &entries {
			if entry.file_type == "dir" {
			    println!("{filename}", filename=entry.path);
			}
		    }
		}
		_ => panic!("Unknown command")
	    }
	},
	"write-tree" => {
	    let src = env::current_dir().unwrap(); 
	    read_dir(src.to_str().unwrap());
	}
	_ => println!("unknown command: {}", args[1])
    }

    Ok(())
}

// struct Tree {
// 	entries: Vec<Entry>,
// 	sha1: String 
// }

// impl Tree {
//     fn new(entries: Vec<Entry>, sha1: String) -> Tree {
// 	let tree = Tree {
// 	    entries,
// 	    sha1 
// 	};
// 	return tree;
//     }
// }

fn read_dir(path: &str) {
    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.file_name() == ".git" {
                    continue; // Skip .git directories
                }

                let ty = entry.file_type();
                match ty {
                    Ok(t) => {
                        if t.is_dir() {
                            println!("Dir: {:?}", entry.file_name());
                            read_dir(entry.path().to_str().unwrap());
                        } else {
                            println!("File: {:?}", entry.file_name());
                        }
                    }
                    Err(e) => println!("Error: {:?}", e),
                }
            }
        }
    } else {
        eprintln!("Error reading directory: {:?}", path);
    }
}
