use clap::{Parser, Subcommand, builder::Str};
use inquire::{self};

#[derive(Parser)]
#[command(version,about,long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New {},
    Test01 {
        #[arg()]
        param01: Option<String>,
    },
}

#[derive(Debug)]
struct Entry {
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
    // Init CLI parser
    let cli = Cli::parse();

    // Check command
    match &cli.command {
        Commands::New {} => new_entry(),
        Commands::Test01 { param01 } => {
            println!("This is the test01 command.");
            println!("{}", test01(param01.as_ref().unwrap()));
        }
    }

    // println!("{}", date.format("%d/%m/%Y").to_string());

    println!("Hello, world!");
}

fn test01(a: &String) -> String {
    // a.clone()
    //     .split(",")
    //     .map(|a: &str| a.trim().replace(" ", "-"))
    //     .collect::<Vec<String>>()
    //     .join(",")
    a.clone().slugify(25)
}

fn new_entry() {
    // Get user input: Title
    let title = inquire::Text::new("Title:")
        .prompt()
        .expect("Title error.")
        .slugify(25);

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
        title,
        date: date.format("%d/%m/%Y").to_string(),
        tags,
    };
    println!("{:?}", &entry);
}

pub fn slugify(phrase: &str, char_limit: usize) -> String {
    let mut slug = String::new();

    // Split by whitespace to handle multiple spaces gracefully
    for word in phrase.split_whitespace() {
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
