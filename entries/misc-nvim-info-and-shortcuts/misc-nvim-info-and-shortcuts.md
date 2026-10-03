+++
entry_name = "misc-nvim-info-and-shortcuts"
title = "Misc nvim info and shortcuts"
date = "26/08/2025"
tags = "nvim,futureref"
+++

- In Normal mode <S-J> joins the current line with the one below it, inserting a space between them.
- '[' and ']' keys are used for prev and next actions respectively. Press either other those keys and explore the which-key menu to explore the options. Keep in mind, the options present will depend on what you're doing. If you're only editing text, you'll have different options. If you're coding you'll have different options. Some examples:
  - [b and ]b for prev buffer and next buffer
  - [f and ]f to jump to prev and next start of function
  - [F and ]F to jump to prev and next end of function
  - [a and ]a to jump to prev and next start of parameter
  - [A and ]A to jump to prev and next end of parameter
  - [<space> and ]<space> to add empty line above and below the cursor
- Viewing notifications: <space>n and <space>sna
- In Normal mode 'o' key will insert a newline below and switch to Insert mode. <S-O> will insert a newline above and switch Insert mode. This is more convenient than, for example, switching Insert mode, moving cursor to the end of the line and then hitting Enter to insert a newline below.
- Hover info (for function,type,class,etc.)
  - Note: This has been tested with Python but most probably applies to other languages as well.
  - When in Insert mode, if you type the name of a function or class and then hit '(' the LSP pops up a floating window with information about the function or class. When in Normal mode, if you want quick info about a variable,function,class,etc. in the code, hover the cursor over the word and hit <S-K>. It'll pop up a floating window showing the info. The info will be more concise, but it's still helpful. In the floating window you can scroll using <C-f> and <C-b>.

