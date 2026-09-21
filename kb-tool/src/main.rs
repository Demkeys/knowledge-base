use clap::{Parser, Subcommand};
use colored::Colorize;
use inquire::{self};
use serde::Serialize;
use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Parser)]
#[command(version,about,long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create new knowledge base entry.
    New {},
    /// (DEBUG ONLY) Test01 subcommand for random testing.
    Test01 {
        #[arg()]
        param01: Option<String>,
    },
    /// (DEBUG ONLY) Test02 subcommand for random testing.
    Test02,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
struct Entry {
    entry_name: String,
    title: String,
    date: String,
    tags: String,
}

trait SlugFrom {
    fn slugify(&self, char_limit: usize) -> String;
}
impl SlugFrom for String {
    fn slugify(&self, char_limit: usize) -> String {
        let mut slug = String::new();

        // Split by whitespace to handle multiple spaces gracefully
        for word in self.as_str().split_whitespace() {
            // Clean the word: lowercase it and keep only alphanumeric characters
            let cleaned_word: String = word
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect();

            if cleaned_word.is_empty() {
                continue;
            }

            // Calculate what the length would be if we added this word
            // (including the hyphen if the slug isn't empty)
            let extra_len = if slug.is_empty() { 0 } else { 1 } + cleaned_word.len();

            if slug.len() + extra_len <= char_limit {
                if !slug.is_empty() {
                    slug.push('-');
                }
                slug.push_str(&cleaned_word);
            } else {
                // Stop adding words if the next one exceeds the limit
                break;
            }
        }

        slug
    }
}

fn main() {
    // Check if entries dir exists or not.
    let entries_dir_path = generate_entries_dir_path();
    check_entries_dir_exists(&entries_dir_path);

    // Init CLI parser
    let cli = Cli::parse();

    // Check command
    match &cli.command {
        Commands::New {} => new_entry(&entries_dir_path),
        Commands::Test01 { param01 } => {
            println!("This is the test01 command.");
            println!("{}", test01(param01.as_ref().unwrap()));
        }
        Commands::Test02 => test02(),
    }

    // println!("{}", date.format("%d/%m/%Y").to_string());

    // println!("Hello, world!");
}

/// A debugging function for scratch code.
fn test01(a: &String) -> String {
    // a.clone()
    //     .split(",")
    //     .map(|a: &str| a.trim().replace(" ", "-"))
    //     .collect::<Vec<String>>()
    //     .join(",")
    a.clone().slugify(25)
}

/// A debugging function for scratch code.
fn test02() {
    let str01 = ".md";

    let str02 = format!("abc{str01}{}", str01);
    println!("{}", &str02);
}

fn new_entry(entries_dir_path: &PathBuf) {
    // Get user input: Title
    let title = inquire::Text::new("Title:").prompt().expect("Title error.");

    // Get user input: Date
    let date = inquire::DateSelect::new("Select a date")
        .prompt()
        .expect("Date error.");

    // Get user input: Tags
    let tags = inquire::Text::new("Tags (comma-seperated, no spaces):")
        .prompt()
        .expect("Tags error.")
        // - Split string by commas into multiple item strings
        // - Trim whitespaces in each item string
        // - Replace spaces with hyphens in each item string
        // - Collect all item strings in Vec<String>
        // - Join all items in Vec<String>, separated by comma.
        // This code takes the long string of comma spearated tags, removes any whitespaces
        // near commas, then replaces spaces in multiword tags with hyphens.
        .split(",")
        .map(|a: &str| a.trim().replace(" ", "-"))
        .collect::<Vec<String>>()
        .join(",");

    // Store data in Entry object
    let entry = Entry {
        entry_name: title.slugify(35),
        title,
        date: date.format("%d/%m/%Y").to_string(),
        tags,
    };

    // println!("{:#?}", &entry);

    write_entry_to_file(&entry, entries_dir_path);
}

// Generate path to entries dir. This path would be different depending on
// whether the program is being run via cargo or as a standalone.
fn generate_entries_dir_path() -> PathBuf {
    // Set entries_dir_path based on whether we are running the program through cargo
    // or as a standalone. Depending on how it is being run, we want to use different
    // mechanisms to generate the path.
    // If running via cargo...
    let entries_dir_path = if let Ok(path) = env::var("CARGO_MANIFEST_DIR") {
        // ... point path to 'test-entries' dir within project dir.
        let mut temp_path = PathBuf::from(path);
        temp_path.push("test-entries");
        temp_path
        // else we assume program is running as standlone.
    } else {
        // Check if binary is running from 'tool' dir. If not, panic.
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
            panic!(
                "{}{}{}",
                "Program is not running from ".red(),
                "'tools'".blue(),
                " dir. Exiting...".red()
            );
        }
        // Generate path to point to 'entries' dir in knowledge base root.
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

// Check if specified entries dir exists. If not, panic.
fn check_entries_dir_exists(entries_dir_path: &Path) {
    // If 'entries' dir doesn't exist, panic.
    if !entries_dir_path.exists() {
        panic!(
            "{}{}{}",
            "Knowledge Base doesn't contain ".red(),
            "'entries'".blue(),
            " dir. Create it!".red()
        );
    } else {
        println!(
            "Knowledge Base entries dir found at: {}",
            entries_dir_path.display().to_string().blue(),
        );
        println!("Proceeding...");
    }
}

fn write_entry_to_file(entry: &Entry, entries_dir_path: &Path) {
    // Set up path for new entry dir
    let entry_dir_path = entries_dir_path.join(&entry.entry_name);

    // Create dir. Dir should not already exist since this is a new entry.
    fs::create_dir(&entry_dir_path).expect("Cannot create dir due to");
    println!("Created entry dir: \t{}", entry_dir_path.display());

    {
        // Create and open file.
        let file_path = entry_dir_path.join(format!("{}.md", entry.entry_name));
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(false)
            .create(true)
            .open(&file_path)
            .unwrap();

        // Write front matter TOML data to file.
        let mut entry_data_str = toml::to_string::<Entry>(entry).unwrap();
        entry_data_str.insert_str(0, "+++\n");
        entry_data_str.push_str("+++");
        file.write_all(entry_data_str.as_bytes()).unwrap();

        println!("Created entry file: \t{}", file_path.display());
    }
}
