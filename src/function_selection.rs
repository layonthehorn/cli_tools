use anyhow::{Context, Result};
use std::path::PathBuf;
// using Options struct from cli interface
use crate::cli_interface::Options;
use crate::print_functions;
use std::fs::read_dir;
use walkdir::WalkDir;

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
        let path_vec;
        if flags.is_rec() {
            path_vec = collect_dir_contents_recursively(&path);
        } else {
           path_vec = collect_dir_contents(&path);
        }
        match flags.get_options() {
            // lists all files
            (true, true) => {
                print_functions::list_hidden_files(path_vec).context("Failed to print results.")?;
            }
            // only lists nonhidden files
            (true, false) => {
                print_functions::list_normal_files(path_vec).context("Failed to print results.")?;
            }
            // shows all files in nonlist format
            (false, true) => {
                print_functions::print_hidden_files(path_vec)
                    .context("Failed to print results.")?;
            }
            // shows nonhidden files in nonlist format
            (false, false) => {
                print_functions::print_normal_files(path_vec)
                    .context("Failed to print results.")?;
            }
        }
    }

    Ok(())
}

fn collect_dir_contents_recursively(path: &PathBuf) -> Vec<PathBuf> {

    let mut dir_list: Vec<PathBuf> = Vec::new();
    let mut file_list: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let dir_path = entry.path();
        if dir_path.is_dir(){
            dir_list.push(PathBuf::from(dir_path));
        } else {
            file_list.push(PathBuf::from(dir_path));
        }
    }
    file_list.append(&mut dir_list);
    file_list
}

// collects all the files in a given directory
fn collect_dir_contents(path: &PathBuf) -> Vec<PathBuf> {
    let mut dir_list: Vec<PathBuf> = vec![path.join("..")];
    let mut file_list: Vec<PathBuf> = Vec::new();
    for entry in read_dir(path).unwrap() {
        match entry {
            Ok(t) => {
                let inner_path = t.path();
                if inner_path.is_dir() {
                    dir_list.push(inner_path.clone());
                } else {
                    file_list.push(inner_path.clone());
                }
            }
            Err(_e) => {}
        }
    }
    file_list.append(&mut dir_list);
    file_list
}
