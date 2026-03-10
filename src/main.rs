use std::{fs::{self, File}, path::{Path, PathBuf}};
use clap::Parser;
use owo_colors::OwoColorize;
use chrono::{DateTime, Utc};

#[derive(Debug, Parser)]
struct Cli {
    path: Option<PathBuf>
}

fn main() {
    let cli = Cli::parse();

    let path = cli.path.unwrap_or(PathBuf::from("."));

    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            get_files(&path).iter()
            .for_each(|f| println!("{:#?}", f));

        } else {
            println!("{}", "provided path does not exist".red())
        }
    } else {
            println!("{}", "error reading directory".red())
    }

    println!()
}

fn get_files(path: &Path) -> Vec<FileMetadata> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                if let Ok(metadata) = fs::metadata(&path) {
                    let file_info = FileMetadata {
                        name: file.file_name()
                            .into_string()
                            .unwrap_or("unknown".into()),
                        _type: if metadata.is_dir() {
                            EntryType::Dir
                        } else {
                            EntryType::File
                        },
                        len_bytes: metadata.len(),
                        modified: if let Ok(modi) = metadata.modified() {
                            let date: DateTime<Utc> = modi.into();
                            format!("{}", date.format("%a %b %e %Y"))
                        } else {
                            String::default()
                        }
                    };
                    data.push(file_info);
                }

                
            }
        }
    }
    data
}

#[derive(Debug)]
enum EntryType {
    File,
    Dir
}

#[derive(Debug)]
struct FileMetadata{
    name: String,
    _type: EntryType,
    len_bytes: u64,
    modified: String
}