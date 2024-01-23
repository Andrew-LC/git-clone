#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use flate2::{self, read::ZlibDecoder, write::ZlibEncoder, Compression};
use std::fs;

mod entry;
use entry::Entry;
mod utils;
use utils::{create_object_path, write_object, hash, decompress, read_dir};

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
		   decompress(&args[3]); 
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
		    let file_content = fs::read_to_string(filename)?;
		    let sha1 = hash(&file_content);

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
	    let command = &args[2];
	    match command.as_str() {
		"--name-only" => {
		    Entry::ls_tree();
		}
		_ => panic!("Unknown command")
	    }
	},
	"write-tree" => {
	    match read_dir() {
		Ok(value) => print!("{:?}", value),
		Err(err) => eprintln!("Err: {:?}", err)
	    }
	}
	_ => println!("unknown command: {}", args[1])
    }

    Ok(())
}
