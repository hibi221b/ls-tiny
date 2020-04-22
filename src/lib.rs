//! **ls-tiny** is a less functional `ls` command
//! it is somewhat richer than when no argument to the ls command is specified. (Because it's a little colored.)

#[macro_use]
extern crate clap;
use clap::{Arg, App};
use colored::Colorize;
use std::path::{Path, PathBuf};

/// The Config structure has four fields
/// * the path of the directory you want to look up
/// * the number of directories
/// * the number of files
/// * the total number of them.
pub struct Config {
    /// directory name given by the first argument
    search_dir: PathBuf,
    /// Number of directories
    dirs: u64,
    /// Number of files
    files: u64,
    /// total entries
    entries: u64
}

/// Define the initial value of the Config structure.
impl Default for Config {
    fn default() -> Self {
        Self {
            search_dir: PathBuf::new(),
            dirs: 0,
            files: 0,
            entries: 0
        }
    }
}


impl Config {
    /// parse the arguments.
    /// If you want more information, such as methods, check out the following links
    /// clap https://crates.io/crates/clap
    fn parse_arguments<'a>() -> clap::ArgMatches<'a> {
        App::new(crate_name!())
            .version(crate_version!())      
            .author(crate_authors!())       
            .about(crate_description!())
            .arg(Arg::with_name("PATH")
                .help("Sets the directory") 
                .required(true)
                .index(1)
            )
            .get_matches()
    }

    /// If an existing directory is specified, the Ok-wrapped Config structure is returned.
    /// Assign the directory specified by the first argument to the search_dir field of the Config structure. 
    /// All other fields are assigned a 0.(call the default method)
    /// 
    /// # Panics 
    /// An error is returned when the first argument is a file or a non-existent directory is specified.
    pub fn new() -> Result<Config, String> {
        let matches = Self::parse_arguments();
        let mut search_dir = PathBuf::new();

        if let Some(path) = matches.value_of("PATH") {
            search_dir.push(path);
        }

        if search_dir.is_file() {
            return Err(format!("error: sets the directory"));
        }
    
        if !search_dir.exists() {
            return Err(format!("error: {} cannot find", search_dir.display()));
        }

        Ok( Config { search_dir, ..Config::default() } )
    }

    /// check the entries in the directory specified by the first argument.
    /// If you want to know more about specifying colors, see the following links:
    /// [colored](https://crates.io/crates/colored)
    pub fn run(&mut self) {
        for dir_entry in self.search_dir.read_dir().expect("cannot read dir") {
    
            self.entries += 1;
    
            if let Ok(entry) = dir_entry {
                let entry_type = if entry.path().is_file() {
                    self.files += 1;
                    "file".green()
                } else {
                    self.dirs += 1;
                    "dir ".cyan()
                };
    
                println!("{}: {} - {}", self.entries , entry_type , get_name(&entry.path()));
            }
        }

        let dir_name = get_name(self.search_dir.as_path()).magenta();
        let entries = self.entries.to_string().magenta();

        println!("\ndir: {}, file: {}", self.dirs.to_string().cyan(), self.files.to_string().green());
        println!("{} directory has {} entries.", dir_name, entries);
    }
}

/// extracts only the name from the path
fn get_name(path: &Path) -> &str {

    if path == Path::new(".") {
        return "current";
    }
    
    path.file_name().unwrap().to_str().unwrap()
}