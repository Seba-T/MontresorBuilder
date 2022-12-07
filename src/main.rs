mod utils;

use ansi_term::{self, Colour};
use clap::Parser;
use std::fs;
use crate::utils::*;

const DEFAULT_PATH: &'static str = "~/Downloads";

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Path where the file will be searched for
    #[clap(short, long, default_value = "~/Downloads")]
    source_path: String,
    /// Path where the project folder will be created
    #[clap(short, long, default_value = "current directory")]
    destination_path: String,
    name: String,
}


fn generate_project() -> Result<String, String> {
    let args = Args::parse();
    let source_path: String = parse_path(&args.source_path);

    let project_name = args.name.split_whitespace().next().unwrap();
    let destination_path: String = if args.destination_path == "current directory" {
        format!("./{}", project_name)
    } else {
        format!("{}/{}", parse_path(&args.destination_path), project_name)
    };
    let mut project_real_names: Vec<String> = Vec::new();
    match fs::read_dir(&source_path) {
        Ok(dir_entries) => {
            project_real_names.append(&mut dir_entries.filter_map(|dir_entry| {
                if let Ok(current_file) = dir_entry.unwrap().file_name().into_string() {
                    if current_file.contains(project_name) {
                        Some(current_file)
                    } else { None }
                } else { None }
            }).collect::<Vec<String>>());
        }
        Err(_) => { return Err("Error while trying to read from the source directory".to_string()); }
    };
    //for now we just use the first match, later you will be able to pick the most fitting match
    return if project_real_names.len() <= 0 {
        Err(format!("No match found for project {}", project_name))
    } else {
        let created_dir = fs::create_dir(&destination_path);
        match created_dir {
            Ok(_) => {
                move_file(&source_path, &destination_path, &project_real_names[0])?;
                //moving the pdf file to the new directory
                let _ = std::env::set_current_dir(&destination_path);

                //now writing the package.json in the new directory
                create_write_json(&project_name)?;

                match fs::File::create(format!("{}.cpp", &project_name)) {
                    Ok(_) => Ok("Success!".to_string()),
                    Err(_) => Err("Error creating cpp file".to_string())
                }
            }
            Err(_) =>
                Err("An error occurred while creating the project :/".to_string())
        }
    };
}


fn main() {
    match generate_project() {
        Ok(val) => { println!("{}", Colour::Green.paint(val)) }
        Err(err) => { println!("{}", Colour::Red.paint(err)) }
    }
}

#[cfg(test)]
mod tests {
    use crate::generate_project;

    #[test]
    fn generate_project_test() {
        generate_project();
    }
}
