use glob::glob;
use std::path::PathBuf;


fn main(){
    // gets current directory or ends if unreadable
    let result = std::env::current_dir();

    // checks if an error was returned
    if result.is_err(){
        println!("Can not read directory.");

    } else {
        let cwd = result.unwrap();
        let match_string = create_pattern(&cwd);
        list_files(match_string);

        };



}


fn create_pattern(path: &PathBuf) -> String {
    let match_string = path.join("*");
    let return_string = match_string.to_str().expect("Could not create pattern string.");
    return_string.to_string()
}

fn list_files(matcher: String){
    for entry in glob(&matcher).unwrap(){
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }

    }
}
