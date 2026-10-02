#![allow(unused_imports)]
// DEBUG ONLY.
use colored::Colorize;
use core::panic;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fmt::format,
    fs::{self, OpenOptions, read},
    io::{self, BufRead, BufReader, Write},
    mem::{size_of, size_of_val},
    ops::Deref,
    path::{Path, PathBuf},
};
use toml;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
struct Entry {
    entry_name: String,
    title: String,
    date: String,
    tags: String,
}

fn main() {
    some01();
}

fn some01() {
    // let entries_dir_path =
    //     Path::new("/home/demkeys/Work/Projects/knowledge-base/kb-tool/test-entries");
    println!("{}", env::current_exe().unwrap().display());
    // let entries_dir_path = env::current_exe()
    //     .unwrap()
    //     .join(Path::new("../../test-entries"));
    // let entries_dir_path = if let Ok(path) = env::var("CARGO_MANIFEST_DIR") {
    //     // ... point path to 'test-entries' dir within project dir.
    //     let mut temp_path = PathBuf::from(path);
    //     temp_path.push("test-entries");
    //     temp_path
    //     // else we assume program is running as standlone.
    // } else {
    //     PathBuf::from("../entries")
    // };

    let (entries_dir_path, gh_path_prefix) = if let Ok(path) = env::var("CARGO_MANIFEST_DIR") {
        // ... point path to 'test-entries' dir within project dir.
        let mut temp_path = PathBuf::from(path);
        temp_path.push("test-entries");
        (temp_path, Path::new("kb-tool/test-entries"))
        // else we assume program is running as standlone.
    } else {
        (PathBuf::from("../entries"), Path::new("../entries"))
    };

    // let entries_dir_path = Path::new("test-entries");
    let mut md_str = String::new();
    println!("entries_dir_path: {}", entries_dir_path.display());
    println!("entries_dir_path is dir: {}", entries_dir_path.is_dir());
    println!("gh_path_prefix: {}", gh_path_prefix.display());

    md_str.push_str("## Knowledge Base Index\n");
    md_str.push_str("---\n\n");
    md_str.push_str(
        "___NOTE: This is an auto-generated index of the entries in the knowledge-base \
        made for the convenience of people viewing the knowledge-base on GitHub. The knowledge-\
        base will eventually be on my website. But for anyone viewing the repo on GitHub, this \
        auto-genereated index should be useful for browsing entries.___\n\n",
    );

    for entry in fs::read_dir(entries_dir_path).unwrap() {
        let entry_dir_path = entry.unwrap().path();
        println!("{}", entry_dir_path.display());
        println!("{}", entry_dir_path.file_name().unwrap().display());
        let entry_file_path = entry_dir_path.join(format!(
            "{}.md",
            entry_dir_path.file_name().unwrap().display()
        ));

        let file = OpenOptions::new()
            .read(true)
            .open(&entry_file_path)
            .unwrap();
        let reader = BufReader::new(file);
        let file_data: String = reader
            .lines()
            .skip(1)
            .take(4)
            .map(|line| format!("{}\n", line.unwrap()))
            .collect();
        let entry_data = toml::from_str::<Entry>(&file_data).unwrap();
        let gh_file_path = gh_path_prefix
            .join(entry_dir_path.file_name().unwrap())
            .join(entry_file_path.file_name().unwrap());

        // Construct md text
        md_str.push_str(format!("- [{}]", entry_data.title).as_str());
        md_str.push_str(format!("({})\n", gh_file_path.to_str().unwrap()).as_str());
        println!("{:#?}", entry_data);
        println!("{:?}", entry_file_path.file_name());
    }

    println!("{}", md_str);
    {
        let kb_index_path = Path::new("../kb-index.md");
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(kb_index_path)
            .unwrap();
        file.write_all(md_str.as_bytes()).unwrap();
    }
}
