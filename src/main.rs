
fn main() {
    // gets current working directory
    // no error handling for now
    let pwd = std::env::current_dir().unwrap();
    println!("The currect dir is {}", pwd.display());

}
