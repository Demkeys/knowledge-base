## Knowledge Base

This is a knowledge base (kb) containing various bits of knowledge that I learn as I go. The knowledge base follows a Leaf Bundle structure - each entry is contained within a directory. In each directory there will be one markdown file containing the kb entry data, and any media files that might be referenced in the markdown file. Entries are managed using 'kb-tool'. See 'kb-tool' section for more info.

### SETUP
- Clone repo and cd into repo dir.

### NOTES
- 'entries' dir should exist in root dir before kb-tool is used. This is where the entries are stored. This is where kb-tool will read/write entries from/to.
- kb-tool should be used from 'tools' dir in root dir.

### USAGE
- To create new knowledge base entry
  - Use ``` tools/kb-tool ```
    - ``` kb-tool new ```
  - Program will give you prompts for data.
  - Enter data and the entry will be created.

### kb-tool
Entries are managed using a dedicated tool called 'kb-tool' located at 'tools/kb-tool'. Each entry is represented by a dir with the entry, a '*.md' file within the dir with the same entry name, and any additional media files that need to be referenced in the md file. The md file contains the entry data.

#### Subcommands:
Use ```kb-tool --help``` to get info about the various subcommands.

- new: Used to create a new kb entry. This mainly creates the directory and file and populates the file with TOML front matter data. You can then open up the md file in your editor and add in whatever data you want to.

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
- [x] new-entry subcommand



