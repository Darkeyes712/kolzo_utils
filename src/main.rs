mod utils;
use clap::{App, Arg};

fn main() {
    let matches = App::new("kolzo_utils")
        .version("1.0")
        .author("Your Name")
        .about("Lists files and directories")
        .arg(
            Arg::with_name("path")
                .short("p")
                .long("path")
                .takes_value(true)
                .help("Specifies the path to list (default is current directory)"),
        )
        .subcommand(App::new("ls"))
        .get_matches();

    let path = matches.value_of("path").unwrap_or(".");
    match utils::common_logic::get_all_items_from_directory(path) {
        Ok(metadata_entries) => utils::ls::print_metadata_entries(metadata_entries),
        Err(err) => eprintln!("Error: {}", err),
    }
}
