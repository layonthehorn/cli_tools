use anyhow::Result;
use std::fs::read_dir;
use std::io::Write;
use std::path::PathBuf;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

// prints all files in a single line
pub fn print_hidden_files(path_list: Vec<PathBuf>) -> Result<()> {
    let bufwtr = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();
    for path in path_list.iter() {
        let base_name = get_file_base_name(&path);
        if path.is_dir() {
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
            write!(&mut buffer, "{}", format!("{} ", base_name))?;
        } else {
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
            write!(&mut buffer, "{}", format!("{} ", base_name))?;
        }
    }

    write!(&mut buffer, "\n")?;
    bufwtr.print(&buffer)?;
    Ok(())
}

// prints visible files in a list format
pub fn print_normal_files(path_list: Vec<PathBuf>) -> Result<()> {
    let bufwtr = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();
    for path in path_list.iter() {
        let base_name = get_file_base_name(&path);
        if !base_name.starts_with(".") {
            if path.is_dir() {
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
                write!(&mut buffer, "{}", format!("{} ", base_name))?;
            } else {
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
                write!(&mut buffer, "{}", format!("{} ", base_name))?;
            }
        }
    }

    write!(&mut buffer, "\n")?;
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
        if !base_name.starts_with(".") {
            write!(&mut buffer, "{} ", get_attributes(path))?;
            if path.is_dir() {
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
                write!(&mut buffer, "{}", format!("{} ", base_name))?;
                write!(&mut buffer, "\n")?;
            } else {
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
                write!(&mut buffer, "{}", format!("{} ", base_name))?;
                write!(&mut buffer, "\n")?;
            }
        }
    }

    bufwtr.print(&buffer)?;
    Ok(())
}

// prints visible files in a list format
pub fn list_hidden_files(path_list: Vec<PathBuf>) -> Result<()> {
    let bufwtr = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();
    for path in path_list.iter() {
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
        let base_name = get_file_base_name(&path);
        write!(&mut buffer, "{} ", get_attributes(path))?;
        if path.is_dir() {
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
            write!(&mut buffer, "{}", format!("{} ", base_name))?;
            write!(&mut buffer, "\n")?;
        } else {
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
            write!(&mut buffer, "{}", format!("{} ", base_name))?;
            write!(&mut buffer, "\n")?;
        }
    }

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

    // gets number of files inside directory
    let number_files = match read_dir(file) {
        Ok(files) => files.count().to_string(),
        Err(_e) => "1".to_string(),
    };
    let file_or_dir = if file.is_dir() { 'D' } else { 'F' };
    // attempts to get metadata if possible
    match file.metadata() {
        // added spacing to the list print for readability
        Ok(meta) => format!(
            "{}{:>3}{:>7},{}",
            file_or_dir,
            number_files,
            meta.len() / convert_kb,
            "KB"
        ),
        Err(_e) => format!("{}", file_or_dir),
    }

    // d            subdirs   owner        group         size  date_modded
    // drwxr-xr-x.  2         layonthehorn layonthehorn  4096  Jul 19 13:13
}
