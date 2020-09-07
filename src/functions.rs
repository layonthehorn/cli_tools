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

// collects all the files in a given directory
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
/*
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
*/