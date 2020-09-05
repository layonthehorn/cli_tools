mod functions;
mod cli_interface;

use clap::{Arg, App, ArgMatches};
use std::path::PathBuf;
use anyhow::{Result, Context};

fn main() -> Result<()>{
    let cli = cli_interface::get_arguments();
    let path = cli_interface::get_directory(cli)?;

    println!("{}",path.display());


    // gets current directory or ends if unreadable
    // checks if an error was returned
    //let cwd = result;
    //let match_string = functions::create_pattern(&cwd)?;
    //functions::list_files(match_string);
    Ok(())
}