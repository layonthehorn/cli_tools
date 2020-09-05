use clap::{Arg, App, ArgMatches};
use anyhow::{Result, Context};
use std::path::PathBuf;

pub fn get_arguments() -> ArgMatches<'static> {
    App::new("List Directories")
        .version("0.1")
        .author("Layonthehorn")
        .about("Lists directories and files")
        // option for formatting
        .arg(Arg::with_name("List")
            .short("l")
            .long("list")
            .help("Lists files in a vertical style"))
        // optional directory to scan
        .arg(Arg::with_name("Directory")
            .help("Directory to scan.")
            // will be index one if it exists
            .index(1)).get_matches()


}
pub fn get_directory(cli_result: ArgMatches) -> Result<PathBuf> {
    let path: PathBuf;
    let directory = cli_result.value_of("Directory").unwrap_or_else(|| { "none" });
    if directory == "none" {
        path = std::env::current_dir().context("Could not open current directory.")?;
    } else {
        let mut temp = PathBuf::new();
        temp.push(directory);
        path = temp;
    };
    Ok(path)
}