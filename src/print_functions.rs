use anyhow::Result;
use std::fs::{read_dir, Metadata};
use std::io::Write;
use std::path::PathBuf;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};
use std::os::unix::fs::MetadataExt;
use users::{get_user_by_uid, get_group_by_gid};

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
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
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
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
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
                buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
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
            buffer.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
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
    let file_or_dir = if file.is_dir() { 'D' } else { 'F' };
    // attempts to get metadata if possible
    match file.metadata() {
        // added spacing to the list print for readability
        Ok(meta) => {
            let user_data = get_user_metadata(&meta);

            format!(
            "{}{:>3} {:>5} {:>5}{:>7},{}",
            file_or_dir,
            //user_data.file_mode,
            get_subdirectory_count(&file),
            user_data.user_name, // to be user id
            user_data.group_name, // to be group id
            meta.len() / convert_kb,
            "KB"
        )},
        Err(_e) => format!("{}", file_or_dir),
    }

    // d            subdirs   owner        group         size  date_modded
    // drwxr-xr-x.  2         layonthehorn layonthehorn  4096  Jul 19 13:13
}

// will hold useful user and file data
#[allow(dead_code)]
struct UserData{
    user_name: String,
    group_name: String,
    file_mode: String,
}

// gets the number of sub directories in a folder
fn get_subdirectory_count(path: &PathBuf) -> String {
    // if its a directory return the count of directories inside it
    let count = if path.is_dir(){
        let mut inner_count = 2;
        for entry in read_dir(path).unwrap() {
            match entry {
                Ok(t) => {
                    if t.path().is_dir(){
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
    count.to_string()

}

fn get_user_metadata(meta: &Metadata) -> UserData {
    let user_id = get_user_by_uid(meta.uid());
    let group_id = get_group_by_gid(meta.gid());
    let user_mode = meta.mode();

    // getting username of file owner
    let user = match &user_id {
        Some(u) =>{u.name().to_str().unwrap_or_else(|| {"DNE"})}
        None => {"DNE"}
    };

    // getting group of file owner
    let group = match &group_id {
        Some(g)=> { g.name().to_str().unwrap_or_else(||{"DNE"})

        },
        None => {"DNE"}
    };
    UserData{
        user_name: user.to_string(),
        group_name: group.to_string(),
        file_mode: create_unix_file_mode(user_mode),
    }
}

// todo: Creation of decent permissions printing
fn create_unix_file_mode(number: u32) -> String {
   number.to_string()
}