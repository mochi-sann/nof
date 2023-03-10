use serde_json::Value;
use std::fs;

pub fn get_scripts() {
    let data = get_package_json("package.json".to_string());
    let scripts = data["scripts"].as_object().unwrap();

    for (key, value) in scripts.iter() {
        println!("{}: {}", key, value);
    }
}

fn get_package_json(file_path: String) -> Value {
    let contents = fs::read_to_string(&file_path).expect("Something went wrong reading the file");

    fs::read_to_string(&file_path).expect("Something went wrong reading the file");
    serde_json::from_str(&contents).unwrap()
}
