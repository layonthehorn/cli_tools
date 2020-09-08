mod cli_interface;
mod function_selection;
mod print_functions;

use anyhow::Result;

fn main() -> Result<()> {
    // getting the input from the cli
    let cli = cli_interface::get_arguments();
    // a struct of the options selected at run time
    let flags = cli_interface::get_options(&cli);
    // gets the directory to scan, fails if cannot read it.
    let path = cli_interface::get_directory(&cli)?;


    function_selection::list_files(&path, &flags)?;

    Ok(())
}
