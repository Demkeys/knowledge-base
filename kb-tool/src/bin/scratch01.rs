#![allow(unused_imports)]
// DEBUG ONLY.
use colored::Colorize;
use core::panic;
use serde::Serialize;
use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    mem::{size_of, size_of_val},
    ops::Deref,
    path::{Path, PathBuf},
};
use toml;

struct MyStruct01;

impl MyStruct01 {
    fn new() -> Self {
        MyStruct01
    }

    fn some_func01(self) -> Self {
        self
    }

    fn some_func02(self) -> Self {
        self
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
struct Entry {
    entry_name: String,
    title: String,
    date: String,
    tags: String,
}

fn main() {
    let str01 = String::from("abc");
    let str01_ref: &str = str01.as_ref();
    let str01_ref01 = str01.deref();

    // let entry = &Entry {
    //     entry_name: String::from("abc"),
    //     title: String::from("abc"),
    //     date: String::from("abc"),
    //     tags: String::from("abc"),
    // };
    // write_entry_to_file(entry);
}

fn generate_entries_dir_path() -> PathBuf {
    let entries_dir_path = if let Ok(path) = env::var("CARGO_MANIFEST_DIR") {
        let mut temp_path = PathBuf::from(path);
        temp_path.push("test-entries");
        temp_path
    } else {
        if env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            != "tools"
        {
            panic!("Not in tools dir");
        }
        let mut temp_path = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        temp_path.push("entries");
        temp_path
    };

    println!("{}", entries_dir_path.display());
    entries_dir_path
}

fn write_entry_to_file(entry: &Entry) {
    let entries_dir_path = generate_entries_dir_path();
    // If 'entries' dir doesn't exist, panic.
    if !entries_dir_path.exists() {
        panic!(
            "{}{}{}",
            "Knowledge Base doesn't contain ".red(),
            "'entries'".blue(),
            " dir. Create it!".red()
        );
    } else {
        println!("Knowledge Base 'entries' dir found. Proceeding...");
    }

    let entry_dir_path = entries_dir_path.join("test-02");
    fs::create_dir(&entry_dir_path).expect("Cannot create dir due to");

    {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            // .open(Path::new("./test-entries/test-01/d"))
            .open(entry_dir_path.join(&entry.entry_name))
            .unwrap();

        // let a = String::from("abc");
        let mut a = toml::to_string::<Entry>(entry).unwrap();
        a.insert_str(0, "+++\n");
        a.push_str("+++");
        // println!("{:?}", a.as_bytes());
        println!("{}", &a);

        // use std::array::from_fn;
        // let b: [u8; 11] = from_fn(|i| i as u8);
        // file.write_all(&b[0..5]).unwrap();
        file.write_all(a.as_bytes()).unwrap();
    }
    // let var01 = env::var("CARGO_MANIFEST_DIR").unwrap();
    // println!("{}", &var01);
}
