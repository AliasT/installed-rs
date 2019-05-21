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

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Package {
    version: String,
    #[serde(default)]
    dependencies: HashMap<String, String>,
    #[serde(default)]
    devDependencies: HashMap<String, String>,
}

fn main() {}

// read package.json
fn get_package(path: &str) -> std::io::Result<Package> {
    // @TODO: relative path must start with ../
    let path = format!("../{}/package.json", path);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let package: Package = serde_json::from_str(contents.as_str()).unwrap();

    Ok(package)
}

#[test]
fn test_diff() -> std::io::Result<()> {
    let parent = "scratch-3.0";
    let package = get_package(parent)?;

    let mut max_len = 0;
    let mut children: Vec<Package> = Vec::new();

    for dep in &package.dependencies {
        let child = get_package(format!("{}/node_modules/{}", parent, dep.0).as_str())?;
        if dep.0.len() > max_len {
            max_len = dep.0.len()
        }
        children.push(child);
    }

    let mut i = 0;
    for dep in &package.dependencies {
        println!(
            "{:>width$}: {:20} -->    {:10}",
            Paint::green(dep.0),
            Paint::blue(dep.1).underline(),
            Paint::red(&children[i].version),
            width = max_len,
        );
        i = i + 1;
    }

    Ok(())
}
