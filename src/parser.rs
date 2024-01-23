use clap::{Parser, Subcommand};

#[derive(Subcommand, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GitCommand {
    Init,                 // Initialize a git repository
    CatFile {
	filepath: String,
	 #[arg(
            short,
            default_value_t = false,
            help = "Pretty-print the contents of the blob"
        )]
        pretty_print: bool,
    }, 
    HashObject {
	filepath: String,
	#[arg(
	    short,
	    default_value_t = false,
	    help = "Write the hash into the object database"
	)]
	pretty_print: bool,
    },
    WriteTree,
    Unknown,
}

#[derive(Parser, Debug)]
#[command(author = "Andrew LC")]
#[command(about = "A git clone written in Rust")]
#[command(version = "0.1.0")]
pub struct Git {
    #[command(subcommand)]
    pub command: GitCommand,
}
