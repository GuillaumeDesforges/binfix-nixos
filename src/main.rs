use std::{fs, io, path::Path};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();

    for file in args.files {
        let file_path = Path::new(file.as_str());
        patch_file(file_path).ok();
    }
}

fn patch_file(file_path: &Path) -> io::Result<()> {
    // ignore if it does not exist
    if !file_path.exists() {
        println!("Ignoring {:?}: No such file or directory", file_path);
        return Ok(());
    }
    // recurse if it is a directory
    if file_path.is_dir() {
        for dir_entry in fs::read_dir(file_path)? {
            let entry_path = dir_entry?.path();
            patch_file(&entry_path)?;
        }
        return Ok(());
    }
    // patch file otherwise
    println!("Patching {:?}", file_path);
    return Ok(());
}
