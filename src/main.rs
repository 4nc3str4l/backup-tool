use clap::{Command, Arg};
use std::{path::PathBuf};


#[tokio::main]
async fn main() {
    let config_path = parse_file_location();
    match config_path {
        Ok(paht) => println!("{}", paht),
        Err(err) => panic!("Invalid path {}", err.to_string()),  
    }
}

fn parse_file_location() -> Result<String, String> {
    let matches = Command::new("backup-tool")
        .arg(
            Arg::new("config")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .default_value("config.json")
        )
        .get_matches();

    let val = matches.get_one::<String>("config").unwrap();
    is_valid_path(val.to_owned())
}

fn is_valid_path(selected_path: String) -> Result<String, String>  {
    if PathBuf::from(&selected_path).exists() {
        Ok(selected_path.to_owned())
    } else {
        Err(format!("The path '{}' does not exist", selected_path))
    }
}