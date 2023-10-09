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
        process_path(file_path).ok();
    }
}

fn process_path(path: &Path) -> io::Result<()> {
    // ignore if it does not exist
    if !path.exists() {
        println!("Ignoring {:?}: No such file or directory", path);
        return Ok(());
    }
    // recurse if it is a directory
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry_path = entry?.path();
            process_path(&entry_path)?;
        }
        return Ok(());
    }
    // TODO check it is an ELF file
    // patch file
    autopatchelf(path)?;
    return Ok(());
}

fn autopatchelf(path: &Path) -> io::Result<()> {
    println!("Patching {:?}", path);
    return Ok(());
}
