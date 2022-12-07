use std::io::Write;

static CPP_DEFAULT_CONTENT: &'static str = r#"
#include <iostream>
#include <fstream>
#include <vector>

using namespace std;

int main()
{
    ifstream in("input.txt");
    ofstream out("output.txt");
    cout << "Hello Montresor!" << endl;
    in.close();
    out.close();
    return 0;
}"#;

pub fn parse_path(path: &str) -> String {
    if path.contains("~") {
        format!("{}{}", home::home_dir().unwrap().to_str().unwrap(), path.split_at(1).1)
    } else {
        path.to_string()
    }
}

pub fn npm_package_generator(executable_name: &String) -> String {
    format!(r#"{{"scripts": {{"start": "/usr/bin/g++ -DEVAL -std=gnu++11 -O2 -pipe -o {} {}.cpp && ./{}"}}}}"#, executable_name, executable_name, executable_name)
}

pub fn move_file(from: &str, to: &str, name: &str) -> Result<String, String> {
    let old_name = format!("{}/{}", parse_path(from), name);
    let new_name = format!("{}/{}", parse_path(to), name);
    //moving the pdf file to the new directory
    match std::fs::rename(old_name, new_name) {
        Ok(_) => Ok("Success".to_string()),
        Err(_) => Err("Error while trying to move the pdf file to the new directory".to_string())
    }
}

pub fn create_write_json(project_name: &str) -> Result<String, String> {
    let package_json = std::fs::OpenOptions::new().create(true).write(true).open("./package.json");
    if let Ok(mut file) = package_json {
        match write!(file, "{}", npm_package_generator(&project_name.to_string())) {
            Ok(_) => Ok("Success!".to_string()),
            Err(_) => Err("Failed to write to package.json".to_string())
        }
    } else {
        Err("Failed to create package.json".to_string())
    }
}

pub fn create_write_cpp(project_name: &str) -> Result<String, String> {
    let cpp_file = std::fs::OpenOptions::new().create(true).write(true).open(format!("{}.cpp", &project_name));
    if let Ok(mut file) = cpp_file {
        match write!(file, "{}", CPP_DEFAULT_CONTENT) {
            Ok(_) => Ok("Success!".to_string()),
            Err(_) => Err("Failed to write to .cpp file".to_string())
        }
    } else {
        Err("Failed to create the cpp file".to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::{fs};
    use std::io::Read;
    use crate::utils::*;

    #[test]
    fn json_creation_test() {
        create_write_json("This is a test").expect("TODO: panic message");
        let json = fs::File::open("package.json");
        assert!(json.is_ok());
        let buffer = &mut String::new();
        json.unwrap().read_to_string(buffer);
        assert_eq!(buffer, &mut npm_package_generator(&"This is a test".to_string()));
    }

    #[test]
    fn cpp_creation_test() {
        create_write_cpp("This is a test").expect("TODO: panic message");
        let cpp = fs::File::open("This is a test.cpp");
        assert!(cpp.is_ok());
        let buffer = &mut String::new();
        cpp.unwrap().read_to_string(buffer);
        assert_eq!(buffer, &mut CPP_DEFAULT_CONTENT.to_string());
    }

    #[test]
    fn parse_path_test() {
        let parsed_path = parse_path(&"~/Desktop/test");
        assert_eq!(parsed_path, "/Users/sebastianotocci/Desktop/test".to_string());
    }

    #[test]
    fn json_formatter_test() {
        let outcome = npm_package_generator(&"montresor".to_string());
        assert_eq!(outcome, r#"{"scripts": {"start": "/usr/bin/g++ -DEVAL -std=gnu++11 -O2 -pipe -o montresor montresor.cpp && ./montresor"}}"#)
    }

    #[test]
    fn file_renaming_test() {
        let outcome = move_file(&"~/Desktop/", &"~/Desktop/test", &"momento_bubuntu.txt");

        let test = fs::read_dir(parse_path("~/Desktop/test")).unwrap().find(|dir| &dir.as_ref().unwrap().file_name().into_string().unwrap() == "momento_bubuntu.txt");
        assert!(test.is_some());
    }
}
