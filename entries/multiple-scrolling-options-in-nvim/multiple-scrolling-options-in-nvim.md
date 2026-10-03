+++
entry_name = "multiple-scrolling-options-in-nvim"
title = "Multiple scrolling options in nvim"
date = "10/08/2025"
tags = "nvim,scrolling,shortcut"
+++

nvim has multiple scrolling options:
Note: C- is Ctrl

There are multiple ways to scroll in nvim. Usually when you're looking at a scrollable buffer, you can scroll using j/k, Up/Down arrow, or even PgUp/PgDwn. But when there are multiple scrollable buffers, the same keymaps don't scroll the other buffers. For example, <Space>sk brings up the Keymap search buffer. There are two scrollable buffers within it. For each keymap, the code used to set it up is shown in a bottom buffer. But you can only use the Up/Down arrow keys to scroll the keymaps list at the top, not the code shown in the buffer below. To scroll the code, you would have to use <C-f> and <C-b>. Another example, in the which-key menu, you use <C-d> and <C-u>. If you're ever in a situation where there are multiple scrollable buffers, try one of these keymaps and see if they allow you to scroll the other buffers.
- <C-d> <C-u>
- <C-f> <C-b>

