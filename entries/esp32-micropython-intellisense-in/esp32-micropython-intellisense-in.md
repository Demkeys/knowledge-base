+++
entry_name = "esp32-micropython-intellisense-in"
title = "ESP32 Micropython intellisense in nvim"
date = "12/09/2025"
tags = "python,nvim,intellisense,futureref"
+++

When working on ESP32 MicroPython projects in nvim, MicroPython intellisense doesn't work out of the box because the modules involved are different from what pyright is expecting.
Solution:
- Install micropython-esp32-stubs using pip. Install in virtual environment.
- Create 'pyproject.toml' file in project root. Check online docs for boilerplate code for that file. Some info might need to be edited to match your project, like Python version, env path, etc. Make the changes and save file. Intellisense should work further from that point.
