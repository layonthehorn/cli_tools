use glob::glob;
use std::path::PathBuf;

fn main() {
    // gets current directory or ends if unreadable
    let pwd = match std::env::current_dir() {
        Ok(path) => path,
        Err(e) => panic!("Could not read directory, {}",e)

    };

    // getting the pattern to match
    let pattern: String = create_pattern(&pwd);
    println!("{}", pattern)


}

fn create_pattern(path: &PathBuf) -> String{
    let match_string = path.join("*");
    let mut return_string = match_string.to_str().unwrap_or_else(||{
        panic!("Could not create pattern string.")
    });
    return_string.to_string()
}
