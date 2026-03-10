use std::{fs::{self}, path::{Path, PathBuf}, fmt};
use clap::Parser;
use owo_colors::{OwoColorize};
use chrono::{DateTime, Utc};
use tabled::{Table, Tabled, settings::{Color, Style, object::{Column, Columns, Rows}}};

#[derive(Debug, Parser)]
struct Cli {
    path: Option<PathBuf>
}

fn main() {
    let cli = Cli::parse();

    let path = cli.path.unwrap_or(PathBuf::from("."));
    
    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            let files = get_files(&path);
            let mut file_table = Table::new(files);
            file_table.with(Style::rounded());
            file_table.modify(Columns::first(), Color::FG_CYAN);
            file_table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);
            println!("{file_table}");
            
        } else {
            println!("{}", "provided path does not exist".red())
        }
    } else {
        println!("{}", "error reading directory".red())
    }




}

fn get_files(path: &Path) -> Vec<FileMetadata> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                if let Ok(metadata) = fs::metadata(&file.path()) {
                    let file_info = FileMetadata {
                        name: file.file_name()
                            .into_string()
                            .unwrap_or("unknown".into()),
                        _type: if metadata.is_dir() {
                            EntryType::Dir
                        } else if metadata.is_file(){
                            EntryType::File
                        } else {
                            EntryType::Unknown
                        },
                        size: metadata.len(),
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
    Dir,
    Unknown
}

impl fmt::Display for EntryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntryType::File => write!(f, "File"),
            EntryType::Dir => write!(f, "Dir"),
            EntryType::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Tabled)]
struct FileMetadata{
    name: String,
    _type: EntryType,
    size: u64,
    modified: String
}