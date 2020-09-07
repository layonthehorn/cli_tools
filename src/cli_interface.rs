use anyhow::{Context, Result};
use clap::{App, Arg, ArgMatches};
use std::path::PathBuf;

pub fn get_arguments() -> ArgMatches<'static> {
    App::new("List Directories")
        .version("0.5")
        .author("Layonthehorn")
        .about("Lists directories and files")
        // option for formatting
        .arg(
            Arg::with_name("List")
                .short("l")
                .long("list")
                .help("Lists files in a vertical style\nAdded extra metadata to print"),
        )
        .arg(
            Arg::with_name("Hidden")
                .short("a")
                .long("all")
                .help("Shows hidden files"),
        )
        // optional directory to scan
        .arg(
            Arg::with_name("Directory")
                .help("Optional directory to scan\nDefaults to current directory")
                // will be index one if it exists
                .index(1),
        )
        .get_matches()
}
pub fn get_directory(cli_result: &ArgMatches) -> Result<PathBuf> {
    let directory = match cli_result.value_of("Directory") {
        Some(path) => {
            let mut temp = PathBuf::new();
            temp.push(path);
            temp
        },
        None => std::env::current_dir().context("Could not open current directory.")?

    };
    Ok(directory)
}

pub fn get_options(cli_result: &ArgMatches) -> Options {
    Options::new(
        cli_result.is_present("List"),
        cli_result.is_present("Hidden"),
    )
}

pub struct Options {
    list: bool,
    all: bool,
}

impl Options {
    fn new(op_list: bool, op_all: bool) -> Options {
        Options {
            list: op_list,
            all: op_all,
        }
    }

    pub fn get_options(&self) -> (bool, bool) {
        (self.list, self.all)
    }
    pub fn list_files(&self) -> bool {
        self.list
    }

    pub fn _all_files(&self) -> bool {
        self.all
    }
}
