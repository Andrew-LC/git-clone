use std::fs;
use clap::Parser;
use git_starter_rust::git_object_utils::Object;
use git_starter_rust::entry;
use git_starter_rust::parser::Git;
use git_starter_rust::parser::GitCommand;

fn main() {
    let git_cli = Git::parse();

    match &git_cli.command {
        GitCommand::Init => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
            println!("Initialized git directory");
        }
        GitCommand::CatFile { filepath, pretty_print } => {
	    if *pretty_print {
		let object = Object::from_sha(&filepath);
		print!("{}", object.unwrap().data);
	    }
        }
	GitCommand::HashObject { filepath, pretty_print } => {
	    if *pretty_print {
		let (sha, location) = Object::hash_object(&filepath).unwrap(); 
		print!("{} {}", sha, location);
	    }
	}
	GitCommand::LSTree { tree_path, .. } => {
	    entry::ls_tree(&tree_path, !true);
	}
	GitCommand::WriteTree { .. } => {
	    todo!()
	}
        GitCommand::Unknown => {
            println!("Unknown command");
        }
    }
}
