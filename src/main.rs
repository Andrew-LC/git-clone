#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use flate2::{self, read::ZlibDecoder, Compression};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use ring::digest::SHA256;

#[allow(dead_code)]
enum GitObj {
    Blob,
    Tree,
    Commit
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
		    let file = &args[3];
		    let contents = fs::read_to_string(file);
		    let sha = ring::digest::digest(&SHA256, contents.unwrap().as_bytes());
		    let compressed_contents = flate2::write::ZlibEncoder::new(contents.unwrap().as_bytes(), flate2::Compression::default());

		    println!("{:?}", sha);
		},
		_ => todo!()
	    }
	}
	_ => println!("unknown command: {}", args[1])
    }

    Ok(())
}
