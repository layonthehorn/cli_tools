use anyhow::{Context, Result};
use glob::glob;
use std::path::PathBuf;
// using Options struct from cli interface
use crate::cli_interface::Options;

pub fn list_files(path: &PathBuf, flags: &Options) -> Result<()> {
    // checks to see if file can be open and if not terminates the program with error message
    std::fs::File::open(path).context(format!("Could not open {} for reading.", path.display()))?;
    let base_name = get_file_base_name(&path);

    if path.is_file() {
        if flags.list_files() {
            let attributes = get_attributes(&path);
            println!("{} {}", attributes, base_name);
        } else {
            println!("{}", base_name)
        }
    } else {
        let pattern = create_pattern(path)?;
        match flags.get_options() {
            // lists all files
            (true, true) => {
                list_hidden_files(pattern)?;
            }
            // only lists nonhidden files
            (true, false) => {
                list_normal_files(pattern)?;
            }
            // shows all files in nonlist format
            (false, true) => {
                print_hidden_files(pattern)?;
            }
            // shows nonhidden files in nonlist format
            (false, false) => {
                print_normal_files(pattern)?;
            }
        }
    }

    Ok(())
}

// attempts to get metadata if possible
// otherwise only lists if the path is a file or a directory
fn get_attributes(file: &PathBuf) -> String {
    let convert_kb = 1000;
    let file_or_dir = if file.is_dir() { 'D' } else { 'F' };
    // attempts to get metadata if possible
    match file.metadata() {
        // added spacing to the list print for readability
        Ok(meta) => format!("{}{:>7},{}", file_or_dir, meta.len()/convert_kb,"KB"),
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
    for entry in glob(&matcher).context("Could not create glob iterator")? {
        match entry {
            Ok(path) => {
                let file_name = get_file_base_name(&path);
                {
                    if !file_name.starts_with('.') {
                        println!("{} {}", get_attributes(&path), get_file_base_name(&path));
                    }
                }
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
    for entry in glob(&matcher).context("Could not create glob iterator")? {
        match entry {
            Ok(path) => {
                let file_name = get_file_base_name(&path);
                if !file_name.starts_with('.') {
                    files.push_str(" ");
                    files.push_str(&file_name);
                }
            }
            Err(_e) => {}
        }
    }
    println!("{}", files.trim());
    Ok(())
}
