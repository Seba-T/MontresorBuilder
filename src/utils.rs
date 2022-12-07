use std::io::Write;

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
    let package_json = std::fs::File::create("./package.json");
    if let Ok(mut file) = package_json {
        match write!(file, "{}", npm_package_generator(&project_name.to_string())) {
            Ok(_) => Ok("Success!".to_string()),
            Err(_) => Err("Failed to write to package.json".to_string())
        }
    } else {
        Err("Failed to create package.json".to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path};
    use crate::utils::{move_file, npm_package_generator, parse_path};

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
