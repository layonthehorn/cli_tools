use anyhow::{Context, Result};
use glob::{glob, glob_with, MatchOptions};
use std::path::PathBuf;
// using Options struct from cli interface
use crate::cli_interface::Options;
use crate::print_functions;
use std::fs::read_dir;

pub fn list_files(path: &PathBuf, flags: &Options) -> Result<()> {
    // checks to see if file can be open and if not terminates the program with error message
    std::fs::File::open(path).context(format!("Could not open {} for reading.", path.display()))?;

    if path.is_file() {
        if flags.list_files() {
            print_functions::list_single_file(&path);
        } else {
            print_functions::print_single_file(&path);
        }
    } else {
        let path_vec = collect_dir_contents(&path);
        let pattern = create_pattern(path)?;
        match flags.get_options() {
            // lists all files
            (true, true) => {
                print_functions::list_hidden_files(path_vec);
                //list_hidden_files(pattern)?;
            }
            // only lists nonhidden files
            (true, false) => {
                print_functions::list_normal_files(path_vec);
                //list_normal_files(pattern)?;
            }
            // shows all files in nonlist format
            (false, true) => {
                print_functions::print_hidden_files(path_vec);
                //print_hidden_files(pattern)?;
            }
            // shows nonhidden files in nonlist format
            (false, false) => {
                print_functions::print_normal_files(path_vec);
                //print_normal_files(pattern)?;
            }
        }
    }

    Ok(())
}

// attempts to get metadata if possible
// otherwise only lists if the path is a file or a directory
fn get_attributes(file: &PathBuf) -> String {
    let convert_kb = 1000;

    // gets number of files inside directory
    let number_files = match read_dir(file) {
        Ok(files) => files.count().to_string(),
        Err(_e) => "1".to_string(),
    };
    let file_or_dir = if file.is_dir() { 'D' } else { 'F' };
    // attempts to get metadata if possible
    match file.metadata() {
        // added spacing to the list print for readability
        Ok(meta) => format!(
            "{}{:>3}{:>7},{}",
            file_or_dir,
            number_files,
            meta.len() / convert_kb,
            "KB"
        ),
        Err(_e) => format!("{}", file_or_dir),
    }

    // d            subdirs   owner        group         size  date_modded
    // drwxr-xr-x.  2         layonthehorn layonthehorn  4096  Jul 19 13:13
}

fn get_file_base_name(path: &PathBuf) -> String {
    let base_name = path
        .file_name()
        .unwrap_or_else(|| std::ffi::OsStr::new(".."));
    // placeholder until better logic is designed
    let final_base = base_name.to_str();
    // attempts to get a printable file base name
    match final_base {
        Some(str) => str.to_string(),
        None => "Error".to_string(),
    }
}

// creates the glob pattern
fn create_pattern(path: &PathBuf) -> Result<String> {
    let match_string = path.join("*");
    let return_string = match_string
        .to_str()
        .context("Could not convert path to usable expression.")?;
    Ok(return_string.to_string())
}

// ls -l
fn list_normal_files(matcher: String) -> Result<()> {
    for entry in glob_with(
        &matcher,
        MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: true,
        },
    )
    .context("Could not create glob iterator")?
    {
        match entry {
            Ok(path) => {
                println!("{} {}", get_attributes(&path), get_file_base_name(&path));
            }

            Err(_e) => {}
        }
    }
    Ok(())
}

// ls -a
fn print_hidden_files(matcher: String) -> Result<()> {
    let mut files = String::new();
    for entry in glob(&matcher).context("Could not create glob iterator")? {
        match entry {
            Ok(path) => {
                let file_name = get_file_base_name(&path);
                files.push_str(" ");
                files.push_str(&file_name);
            }
            Err(_e) => {}
        }
    }
    println!("{}", files.trim());
    Ok(())
}

// ls -al
fn list_hidden_files(matcher: String) -> Result<()> {
    for entry in glob(&matcher).context("Could not create glob iterator")? {
        match entry {
            Ok(path) => println!("{} {}", get_attributes(&path), get_file_base_name(&path)),

            Err(_e) => {}
        }
    }

    Ok(())
}

// ls
fn print_normal_files(matcher: String) -> Result<()> {
    let mut files = String::new();
    for entry in glob_with(
        &matcher,
        MatchOptions {
            case_sensitive: false,
            require_literal_separator: false,
            require_literal_leading_dot: true,
        },
    )
    .context("Could not create glob iterator")?
    {
        match entry {
            Ok(path) => {
                let file_name = get_file_base_name(&path);
                files.push_str(" ");
                files.push_str(&file_name);
            }
            Err(_e) => {}
        }
    }
    println!("{}", files.trim());
    Ok(())
}

fn collect_dir_contents(path: &PathBuf) -> Vec<PathBuf>{
    let mut path_list: Vec<PathBuf> = vec![path.join(".."), path.join(".")];
    for entry in read_dir(path).unwrap(){
        match entry{
            Ok(T) =>{
                path_list.push(T.path());
                //println!("{}", T.path().display());
            },
            Err(_E) => {}
        }

        }
    path_list
    }