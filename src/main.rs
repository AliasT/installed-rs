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
    dependencies: HashMap<String, String>,
    devDependencies: HashMap<String, String>,
}

fn main() {}

// read package.json
fn get_package(path: String) -> std::io::Result<Package> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let package: Package = serde_json::from_str(contents.as_str()).unwrap();

    Ok(package)
}

#[test]
fn test_diff() -> std::io::Result<()> {
    let path = String::from("../scratch-3.0/package.json");
    let package = get_package(path)?;
    for dep in package.dependencies {
        println!(
            "{}: {}",
            Paint::green(dep.0),
            Paint::blue(dep.1).underline()
        );
    }
    Ok(())
}
