use anyhow::Result;
use chrono::{Local, TimeZone};
use std::fs::{read_dir, Metadata};
use std::io::Write;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
use users::{get_group_by_gid, get_user_by_uid};

// prints all files in a single line
pub fn print_hidden_files(path_list: Vec<PathBuf>) -> Result<()> {
    let bufwtr = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();

    // printing loop
    for path in path_list.iter() {
        let base_name = get_file_base_name(&path);
        if path.is_dir() {
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
            write!(&mut buffer, "{}", format!("{} ", base_name))?;
        } else {
            // if the file is executable it colors it green
            if is_executable(&path) {
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            } else {
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
            }
            write!(&mut buffer, "{}", format!("{} ", base_name))?;
        }
    }

    writeln!(&mut buffer)?;
    bufwtr.print(&buffer)?;
    Ok(())
}

// prints visible files in a list format
pub fn print_normal_files(path_list: Vec<PathBuf>) -> Result<()> {
    let bufwtr = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();

    // printing loop
    for path in path_list.iter() {
        let base_name = get_file_base_name(&path);
        if !base_name.starts_with('.') {
            if path.is_dir() {
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
                write!(&mut buffer, "{}", format!("{} ", base_name))?;
            } else {
                // if the file is executable it colors it green
                if is_executable(&path) {
                    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                } else {
                    buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
                }
                write!(&mut buffer, "{}", format!("{} ", base_name))?;
            }
        }
    }

    writeln!(&mut buffer)?;
    // final print
    bufwtr.print(&buffer)?;
    Ok(())
}

// prints all files in a listing format
pub fn list_normal_files(path_list: Vec<PathBuf>) -> Result<()> {
    let bufwtr = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();
    for path in path_list.iter() {
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        let base_name = get_file_base_name(&path);
        if !base_name.starts_with('.') {
            write!(&mut buffer, "{} ", get_attributes(path))?;
            if path.is_dir() {
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
                write!(&mut buffer, "{}", format!("{} ", base_name))?;
                writeln!(&mut buffer)?;
            } else {
                // if the file is executable it colors it green
                if is_executable(&path) {
                    buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
                } else {
                    buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
                }
                write!(&mut buffer, "{}", format!("{} ", base_name))?;
                writeln!(&mut buffer)?;
            }
        }
    }

    // final print
    bufwtr.print(&buffer)?;
    Ok(())
}

// prints visible files in a list format
pub fn list_hidden_files(path_list: Vec<PathBuf>) -> Result<()> {
    let bufwtr = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();

    // printing loop
    for path in path_list.iter() {
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        let base_name = get_file_base_name(&path);
        write!(&mut buffer, "{} ", get_attributes(path))?;
        if path.is_dir() {
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
            write!(&mut buffer, "{}", format!("{} ", base_name))?;
            writeln!(&mut buffer)?;
        } else {
            // if the file is executable it colors it green
            if is_executable(&path) {
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
            } else {
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
            }
            write!(&mut buffer, "{}", format!("{} ", base_name))?;
            writeln!(&mut buffer)?;
        }
    }

    // final print
    bufwtr.print(&buffer)?;
    Ok(())
}

// prints the information about a single file
pub fn print_single_file(path: &PathBuf) {
    println!("{}", get_file_base_name(&path));
}

// lists the information about a single file
pub fn list_single_file(path: &PathBuf) {
    println!("{} {}", get_attributes(&path), get_file_base_name(&path));
}

// gets the base name of the file at the end of a path
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

// gets the attributes of a file
fn get_attributes(file: &PathBuf) -> String {
    let convert_kb = 1000;
    let file_or_dir = if file.is_dir() { 'd' } else { '-' };
    // attempts to get metadata if possible
    match file.metadata() {
        // added spacing to the list print for readability
        Ok(meta) => {
            let user_data = get_user_metadata(&meta);

            format!(
                "{}{} {:>5} {:>10} {:>5}{:>7},{} {}",
                file_or_dir,
                user_data.file_mode,
                // returns ? if unable to look into directory
                get_subdirectory_count(&file).unwrap_or_else(|_| { "?".to_string() }),
                user_data.user_name,  // to be user id
                user_data.group_name, // to be group id
                meta.len() / convert_kb,
                "KB",
                get_last_change(&meta)
            )
        }
        Err(_e) => format!("{}", file_or_dir),
    }

    // d            subdirs   owner        group         size  date_modded
    // drwxr-xr-x.  2         layonthehorn layonthehorn  4096  Jul 19 13:13
}

// will hold useful user and file data
struct UserData {
    user_name: String,
    group_name: String,
    file_mode: String,
}

// gets the number of sub directories in a folder
fn get_subdirectory_count(path: &PathBuf) -> Result<String> {
    // if its a directory return the count of directories inside it
    let count = if path.is_dir() {
        let mut inner_count = 2;
        for entry in read_dir(path)? {
            match entry {
                Ok(t) => {
                    if t.path().is_dir() {
                        inner_count += 1;
                    }
                }
                Err(_e) => {}
            }
        }
        inner_count
    } else {
        // if its a file return one
        1
    };
    Ok(count.to_string())
}

fn get_user_metadata(meta: &Metadata) -> UserData {
    let user_id = get_user_by_uid(meta.uid());
    let group_id = get_group_by_gid(meta.gid());
    let user_mode = meta.mode();

    // getting username of file owner
    let user = match &user_id {
        Some(u) => u.name().to_str().unwrap_or_else(|| "DNE"),
        None => "DNE",
    };

    // getting group of file owner
    let group = match &group_id {
        Some(g) => g.name().to_str().unwrap_or_else(|| "DNE"),
        None => "DNE",
    };
    UserData {
        user_name: user.to_string(),
        group_name: group.to_string(),
        file_mode: create_unix_file_mode(user_mode),
    }
}

// turns unix mode into readable format
fn create_unix_file_mode(number: u32) -> String {
    let mut base_8_mode = format!("{:o}", number);
    let end = base_8_mode.len();
    let start = base_8_mode.len() - 3;
    // getting permissions slice
    base_8_mode = base_8_mode[start..end].to_string();
    let mut print_value = String::new();
    for number in base_8_mode.chars() {
        match number {
            '0' => &print_value.push_str("---"),
            '1' => &print_value.push_str("--x"),
            '2' => &print_value.push_str("-w-"),
            '3' => &print_value.push_str("-wx"),
            '4' => &print_value.push_str("r--"),
            '5' => &print_value.push_str("r-x"),
            '6' => &print_value.push_str("rw-"),
            '7' => &print_value.push_str("rwx"),
            // should never happen but I want to know if it does.
            _ => unreachable!(),
        };
    }
    print_value
}

// checks if the first bit is a 7
fn is_executable(file: &PathBuf) -> bool {
    match file.metadata() {
        Ok(meta) => {
            let permissions = format!("{:o}", meta.mode());
            let first_bit = permissions
                .chars()
                .nth(permissions.len() - 3)
                .unwrap_or_else(|| '0');
            match first_bit {
                '7' => true,
                '1' => true,
                '3' => true,
                '5' => true,
                _ => false,
            }
        }
        Err(_e) => false,
    }
}

// gets last time of modification
fn get_last_change(meta: &Metadata) -> String {
    let dt = Local.timestamp(meta.mtime(), 0);
    /*
    %b - Abbreviated month name. Always 3 letters.
    %a - Abbreviated weekday name. Always 3 letters.
    %e - Day number (01--31), space-padded to 2 digits.
    %r - Hour-minute-second format in 12-hour clocks.
    */
    let formatted = dt.format("%b %a %e %r");
    format!("{}", formatted)
}
