use std::path::PathBuf;
use glob::glob;
use anyhow::{Result, Context};




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