use std::fs;

extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter, Yaml};

pub fn create_action_list_file(name: String) {
    let mut path = String::from("profiles/");
    path.push_str(&name); 

    let doc = &Yaml::Array(vec![Yaml::String(String::from("mkdir example")), Yaml::String(String::from("./script.ps1"))]);
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap(); // dump the YAML object to a String
    }

    fs::write(path, out_str[..].as_bytes());
}