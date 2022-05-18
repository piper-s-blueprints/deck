use std::fs;
use std::path::Path;

extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter, Yaml};

pub fn create_action_list_file(name: &String) {
    let doc = &Yaml::Array(vec![Yaml::String(String::from("mkdir example")), Yaml::String(String::from("./script.ps1"))]);
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap(); // dump the YAML object to a String
    }

    fs::write(get_filepath(name), out_str[..].as_bytes());
}

pub fn check_action_list_file(name: &String) -> bool {
    Path::new(&get_filepath(name)).exists()
}

pub fn get_action_list(name: &String) -> String {
    let contents = fs::read_to_string(get_filepath(name))
        .expect("Unable to read file");

    let doc = &YamlLoader::load_from_str(contents.as_str()).unwrap()[0];
    
    let mut action_list = String::from("");
    for action in doc.as_vec().unwrap() {
        let mut action_str = String::from(action.as_str().unwrap());
        action_str.push_str("\n");

        action_list.push_str(action_str.as_str());
    }

    action_list
}

fn get_filepath(name: &String) -> String {
    let mut path = String::from("profiles/");
    path.push_str(name);
    path
}