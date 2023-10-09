use std::{error::Error, fs, io, path::Path};

use clap::Parser;
use elf::{endian::AnyEndian, ElfBytes};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();

    for file in args.files {
        let file_path = Path::new(file.as_str());
        process_path(file_path).expect(format!("Failed to process {}", file).as_str());
    }
}

fn process_path(path: &Path) -> Result<(), Box<dyn Error>> {
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
    let file_data = std::fs::read(path)?;
    let file = ElfBytes::<AnyEndian>::minimal_parse(file_data.as_slice());
    if file.is_err() {
        println!("Ignoring {:?}: not a dynamic ELF executable", path);
        return Ok(());
    }
    let is_dynamic_executable = file?.section_header_by_name(".interp")?;
    if is_dynamic_executable.is_none() {
        println!("Ignoring {:?}: not a dynamic ELF executable", path)
    }
    // patch file
    autopatchelf(path)?;
    return Ok(());
}

fn autopatchelf(path: &Path) -> io::Result<()> {
    println!("Patching {:?}", path);
    return Ok(());
}
