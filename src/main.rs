// Prod: #![forbid(warnings)] & #![forbid(clippy::unwrap_used)]
// Dev: #![allow(warnings)]
#![allow(warnings)]

use std::env;
use std::process::Command;
use std::fs::{remove_dir_all, write, create_dir_all};

fn main() {
    let argv: Vec<String> = env::args().collect();

    let _output = Command::new("cargo")
        .args(["new", argv[1].as_str()])
        .output()
        .expect(r#"Error executing "cargo new""#);

    // let unparsed_output = read_to_string(format!("{}/.rs/Cargo.toml", env::var("HOME").unwrap())).unwrap();
    let unparsed_output = String::from_utf8(include_bytes!("../template/Cargo.toml").to_vec()).expect("Error converting byte stream in binary to string (try building again)");
    let output = format(unparsed_output, argv[1].clone());
    write(format!("{}/Cargo.toml", argv[1]), output).expect("Error writing Cargo.toml");

    // let unparsed_output = read_to_string(format!("{}/.rs/main.rs", env::var("HOME").unwrap())).unwrap();
    let unparsed_output = String::from_utf8(include_bytes!("../template/main.rs").to_vec()).expect("Error converting byte stream in binary to string (try building again)");
    let output = format(unparsed_output, argv[1].clone());
    write(format!("{}/src/main.rs", argv[1]), output).expect("Error writing src/main.rs");

	// let output = read_to_string(format!("{}/.rs/cp.code-workspace", env::var("HOME").unwrap())).expect("issue workspace");
    let output = String::from_utf8(include_bytes!("../template/cp.code-workspace").to_vec()).expect("Error converting byte stream in binary to string (try building again)");
    write(format!("{}/{}.code-workspace", argv[1], argv[1]), output).expect("Error writing VS Code Workspace");

    // ./.idea/runConfigurations/Run.xml
    create_dir_all(format!("{}/.idea/runConfigurations/", argv[1])).expect("Error creating .idea directory");
    // let output = read_to_string(format!("{}/.rs/Run.xml", env::var("HOME").unwrap())).expect("issue workspace");
    let output = String::from_utf8(include_bytes!("../template/Run.xml").to_vec()).expect("Error converting byte stream in binary to string (try building again)");
    write(format!("{}/.idea/runConfigurations/Run.xml", argv[1]), output).expect("Error writing .idea/runConfigurations/Run.xml");

    let output = String::from_utf8(include_bytes!("../template/Format.xml").to_vec()).expect("Error converting byte stream in binary to string (try building again)");
    write(format!("{}/.idea/runConfigurations/Format.xml", argv[1]), output).expect("Error writing .idea/runConfigurations/Run.xml");

    let output = String::from_utf8(include_bytes!("../template/Clippy.xml").to_vec()).expect("Error converting byte stream in binary to string (try building again)");
    write(format!("{}/.idea/runConfigurations/Clippy.xml", argv[1]), output).expect("Error writing .idea/runConfigurations/Run.xml");

    remove_dir_all(format!("{}/.git/", argv[1])).unwrap_or_else(|_| println!("couldn't delete .git"));
}

fn format(s: String, name: String) -> String {
    let out_iter: Vec<&str> = s.split('`').collect();
    let mut out = String::new();
    out_iter.iter().take(out_iter.len() - 1).for_each(|part| {
        out.push_str(part);
        out.push_str(&name);
    });
    out.push_str(out_iter.last().expect("Last item not available"));
    out
}
