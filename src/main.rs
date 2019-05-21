extern crate serde;
extern crate serde_json;
extern crate yansi;

// std
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

// lib
use serde::{Deserialize, Serialize};
use yansi::Paint;

#[derive(Deserialize, Serialize, Debug)]
struct Package {
    version: String,

    #[serde(default)]
    dependencies: HashMap<String, String>,

    #[serde(default)]
    devDependencies: HashMap<String, String>,
}

impl Package {
    // TODO:
    fn diff(parent: &str, file: String) {
        let package = Package::new(parent);

        let mut max_len = 0;
        let mut children: Vec<Package> = Vec::new();

        for dep in &package.dependencies {
            let child = Package::new(format!("{}/node_modules/{}", parent, dep.0).as_str());
            if dep.0.len() > max_len {
                max_len = dep.0.len()
            }
            children.push(child);
        }

        let mut i = 0;

        fn output_line(name: String, declare: String, installed: String, max: usize) {
            println!(
                "{:>width$}: {:20} -->    {:10}",
                Paint::green(name),
                Paint::blue(declare).underline(),
                Paint::red(installed),
                width = max,
            );
        }

        let mut flag = false;
        for dep in &package.dependencies {
            if file == String::from("*") {
                output_line(
                    dep.0.to_string(),
                    dep.1.to_string(),
                    children[i].version.clone(),
                    max_len,
                );
                i = i + 1;
            } else if (dep.0.to_string() == file && flag == false) {
                flag = true;
                output_line(
                    dep.0.to_string(),
                    dep.1.to_string(),
                    children[i].version.clone(),
                    0,
                );
            }
        }
    }

    // represetation of package
    // read package.json
    fn new(path: &str) -> Package {
        let path = format!("{}/package.json", path);
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents);

        let package: Package = serde_json::from_str(contents.as_str()).unwrap();

        package
    }
}

fn main() {
    // command line args
    // use current dir as default
    let package_dir = match std::env::args().nth(1) {
        Some(path) => path,
        None => String::from("."),
    };

    let module_name = match std::env::args().nth(2) {
        Some(file) => file,

        // compare ass packages
        None => String::from("*"),
    };

    Package::diff(package_dir.as_str(), module_name)
}

#[test]
fn test_diff() {
    Package::diff("../scratch-3.0", String::from(""))
}
