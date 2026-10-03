+++
entry_name = "setting-default-app-for-mimetypes"
title = "Setting default app for mimetypes in Ubuntu"
date = "03/10/2026"
tags = "ubuntu,file,type,mime,application,desktop"
+++

Scenario: You have a terminal-based workflow. You used frogmouth to open up 'test01.txt'. frogmouth is markdown viewer, so when you try to open some other file with it, it invokes some other process that use the default settings to open the file. What ends up happening is the system tries to open the file with calibre, taking you out of your terminal workflow and possibly causing a lag spike.

To avoid the above situation you need to identify the MIME type of the file, then set the default app used to open said MIME type.

To query MIME type:
- xdg-mime query filetype test01.txt
  - Result will be text/plain.
- xdg-mime query filetype test01.md
  - Result will be text/markdown.

To change default application for MIME type:
- xdg-mime default nvim.desktop text/plain
  - This will set nvim as the default app to launch text/plain MIME type.
  - You can do the same for other MIME types.
  - nvim.desktop must already exist at '~/.local/share/applications/'. If it doesn't exist, generate it using Gemini, ChatGPT or whatever. This file can be opened and read and as text file. If the app is supposed to open in the terminal make sure 'Terminal=true' is mentioned in the file. The file follows a standard format that is defined for '.desktop' files, so it's much simpler to just describe the file to an LLM and have it generate the file for you.

A full list of the MIME types and their associated apps can be found at '~/.config/mimeapps.list'.

To test if a given MIME type is opening with it's associated app:
- xdg-open test01.txt
  - In this case the file should open in nvim since we set the default app to 'nvim.desktop' above.
