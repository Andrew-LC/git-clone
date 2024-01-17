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

fn create_object_path(git_path: &str, sha1: &Digest) -> anyhow::Result<PathBuf> {
    let hex_hash = sha1.as_ref().iter().map(|b| format!("{:02x}", b)).collect::<String>();
    let (dir, file) = hex_hash.split_at(2);

    let object_path = Path::new(git_path)
        .join("objects")
        .join(dir);

    // Create directories if they don't exist
    fs::create_dir_all(&object_path)?;

    Ok(object_path.join(file))
}

// Function to compress and write the file content to the object path
fn write_object(object_path: &Path, content: String) -> anyhow::Result<()> {
    let file = File::create(object_path)?;
    let mut encoder = ZlibEncoder::new(file, Compression::default());

    encoder.write_all(content.as_bytes())?;
    encoder.finish()?;
    
    Ok(())
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
		    let mut raw_content = String::new();
		    let (path, file) = &args[3].split_at(2);
		    let file_location = Path::new(&git_object_path).join(path).join(file);

		    if file_location.exists() {
			ZlibDecoder::new(File::open(file_location)?)
                            .read_to_string(&mut raw_content)?;
			let prettyfy_content = raw_content
                            .split_once("\0")
                            .unwrap()
                            .1
                            .trim_end()
                            .trim_end();
			println!("{}", prettyfy_content.trim_end())
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
		    let file_content = fs::read_to_string(filename)?;
		    let sha1 = ring::digest::digest(&SHA256, file_content.as_bytes());
		    let git_object_path = ".git/objects/";

		    // Create the object path based on the SHA1 hash
		    let object_path = create_object_path(git_object_path, &sha1)?;

		    // Compress and write the file content to the object path
		    write_object(&object_path, file_content)?;

		    println!("Object successfully written to: {:?}", object_path);
		},
		_ => todo!()
	    }
	}
	_ => println!("unknown command: {}", args[1])
    }

    Ok(())
}
