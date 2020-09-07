use std::path::PathBuf;
use std::fs::read_dir;

// prints all files in a single line
pub fn print_hidden_files(path_list: Vec<PathBuf>) {
    todo!()

}

// prints visiable files in a list format
pub fn print_normal_files(path_list: Vec<PathBuf>) {
    todo!()
}

// prints all files in a listing format
pub fn list_normal_files(path_list: Vec<PathBuf>) {
    todo!()
}

// prints visible files in a list format
pub fn list_hidden_files(path_list: Vec<PathBuf>) {
    todo!()
}

// prints the information about a single file
pub fn print_single_file(path:&PathBuf) {
    println!("{}",get_file_base_name(&path));
}

// lists the information about a single file
pub fn list_single_file(path: &PathBuf) {
    println!("{} {}",get_attributes(&path) , get_file_base_name(&path));
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
