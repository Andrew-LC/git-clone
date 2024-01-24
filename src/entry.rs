use flate2::read::ZlibDecoder;
use std::io::Read;
use std::path::Path;
use anyhow::Result;
use std::fs;


#[derive(Debug)]
#[allow(dead_code)]
pub struct Entry {
    pub mode: String,
    pub filename: String,
    pub sha: [u8; 20],
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
        let sha: [u8; 20] = rest[sep + 1..sep + 21].try_into().unwrap();
        rest = &rest[sep + 21..];
        children.push(Entry {
            mode,
            filename,
            sha,
        });
    }

    Ok(children)
}

