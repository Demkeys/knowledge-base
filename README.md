## Knowledge Base

This is a knowledge base (kb) containing various bits of knowledge that I learn as I go. The knowledge base follows a Leaf Bundle structure - each entry is contained within a directory. In each directory there will be one markdown file containing the kb entry data, and any media files that might be referenced in the markdown file. Entries are managed using 'kb-tool'. See 'kb-tool' section for more info.

### SETUP
- Clone repo and cd into repo dir.

### NOTES
- 'entries' dir should exist in root dir before kb-tool is used. This is where the entries are stored. This is where kb-tool will read/write entries from/to.
- kb-tool should be used from 'tools' dir in root dir.
- When adding a new entry to knowledge-base remember to generate a new kb-index file.

### USAGE
- To create new knowledge base entry: ```tools/kb-tool new```
  - Program will give you prompts for data.
  - Enter data and the entry will be created.
- To generate kb-index: ```tools/kb-tool generate-kb-index```
  - Program will generate a kb-index file containing links to the markdown files for each entry. This file provides a convenient index for anyone viewing the knowledge-base repo on GitHub, so they can browse entries.

### kb-tool
Entries are managed using a dedicated tool called 'kb-tool' located at 'tools/kb-tool'. Each entry is represented by a dir with the entry, a '*.md' file within the dir with the same entry name, and any additional media files that need to be referenced in the md file. The md file contains the entry data.

#### Subcommands:
Use ```kb-tool --help``` to get info about the various subcommands.

- new: Used to create a new kb entry. This mainly creates the directory and file and populates the file with TOML front matter data. You can then open up the md file in your editor and add in whatever data you want to.
- generate-kb-index: Used to generate a kb-index file. This is a markdown file containing links to each entry. This file is the convenience of people viewing the knowledge-base repo on GitHub.

---

### DEVELOPER NOTES
- 'test-entries' dir should exist in 'kb-tools' dir. During development, entries get written to this directory.
- To test program during development use ```cargo run --bin kb-tool```.
- To release program create release build and copy build to 'tools' dir:
```
cargo build --bin kb-tool --release && cp target/release/kb-tool ../tools/kb-tool
```

### TODO
- [x] Create kb-tool. This tool will have multiple subcommands
- [x] new subcommand
- [ ] generate-index subcommand
  - Generates a 'kb-index.md' file containing links for each kb entry, all in one place for the user to click through. This is mainly so users viewing the repo on GitHub have a way to list out all the entries and select the one they want.

