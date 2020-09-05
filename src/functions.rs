use std::path::PathBuf;
use anyhow::{Result, Context};
// using Options struct from cli interface
use crate::cli_interface::Options;


pub fn list_files(path: &PathBuf, flags: &Options) -> Result<()> {
    // checks to see if file can be open and if not terminates the program with error message
    std::fs::File::open(path).context("Could not open for reading.")?;
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
        todo!()


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



/*
pub fn create_pattern(path: &PathBuf) -> Result<String> {
    let match_string = path.join("*");
    let return_string = match_string.to_str().context("Could not convert path to usable expression.")?;
    Ok(return_string.to_string())
}

pub fn list_files(matcher: String) -> Result<()>{
    for entry in glob(&matcher).unwrap(){
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(())
}
*/