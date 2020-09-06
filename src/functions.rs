use std::path::PathBuf;
use anyhow::{Result, Context};
use glob::glob;
// using Options struct from cli interface
use crate::cli_interface::Options;


pub fn list_files(path: &PathBuf, flags: &Options) -> Result<()> {
    // checks to see if file can be open and if not terminates the program with error message
    std::fs::File::open(path).context(format!("Could not open {} for reading.",path.display()))?;
    let base_name = get_file_base_name(&path);


    if path.is_file(){
        if flags.list_files(){
            let attributes = get_attributes(&path).context("Could not gather attributes for printing.")?;
            println!("{} {}", attributes, base_name);
        }
        else {
            println!("{}", base_name)
        }

    } else {
        let pattern = create_pattern(path)?;
        match flags.get_options(){
            // lists all files
            (true, true) => {
                list_hidden_files(path)?;
            },
            // only lists nonhidden files
            (true, false) => {
                list_normal_files(pattern)?;
            },
            // shows all files in nonlist format
            (false, true) => {
                print_hidden_files(pattern)?;
            },
            // shows nonhidden files in nonlist format
            (false, false) => {
                print_files(pattern)?;

            },

        }

    }

    Ok(())
}

fn get_attributes(file: &PathBuf) -> Result<String> {
    let file_meta_data = file.metadata()?;
    let file_or_dir;
    if file.is_dir(){
        file_or_dir = 'D';
    } else{
        file_or_dir = 'F';
    }

    // d            subdirs   owner        group         size  date_modded
    // drwxr-xr-x.  2         layonthehorn layonthehorn  4096  Jul 19 13:13
    Ok(format!("{} {}", file_or_dir, file_meta_data.len()))

}

fn get_file_base_name(path: &PathBuf) -> String {
    let base_name = path.file_name().unwrap_or_else(|| {std::ffi::OsStr::new("..")});
    // placeholder until better logic is designed
    let final_base = base_name.to_str().unwrap();
    final_base.to_string()

}

fn create_pattern(path: &PathBuf) -> Result<String> {
    let match_string = path.join("*");
    let return_string = match_string.to_str().context("Could not convert path to usable expression.")?;
    Ok(return_string.to_string())
}

// ls -l
fn list_normal_files(matcher: String) -> Result<()>{
    for entry in glob(&matcher).unwrap(){
        match entry {
            Ok(path) => {
                let file_name = get_file_base_name(&path);{
                if file_name.chars().nth(0) != Some('.'){
                    println!("{} {}", get_attributes(&path)?, get_file_base_name(&path) );
                } }

                },

            Err(_e) => {},
        }
    }
    Ok(())
}

// ls -a
fn print_hidden_files(matcher: String) -> Result<()>{
    let mut files = String::new();
    for entry in glob(&matcher).unwrap(){
        match entry {
            Ok(path) => {
                let file_name = get_file_base_name(&path);
                files.push_str(" ");
                files.push_str(&file_name);
            },
            Err(_e) => {},
        }
    }
    print!("{}",files);
    Ok(())
}

// ls -al
fn list_hidden_files(path: &PathBuf) -> Result<()>{
    for entry in std::fs::read_dir(path)?{
        match entry {
            Ok(path) => {
                println!("{} {}",get_attributes(&path.path())? , get_file_base_name(&path.path()))

            }
            Err(_e) => {}
        }

    }
    Ok(())
}

// ls
fn print_files(matcher: String) -> Result<()>{
    let mut files = String::new();
    for entry in glob(&matcher).unwrap(){
        match entry {
            Ok(path) => {
                let file_name = get_file_base_name(&path);
                if file_name.chars().nth(0) != Some('.'){
                    files.push_str(" ");
                    files.push_str(&file_name);
                }
            },
            Err(_e) => {},
        }
    }
    print!("{}",files);
    Ok(())
}
/*
match path.to_str(){
Some(string) => {
files.push_str(" ");
files.push_str(string);
},

None => {
files.push_str(" ");
files.push_str("Error")
}
}
*/