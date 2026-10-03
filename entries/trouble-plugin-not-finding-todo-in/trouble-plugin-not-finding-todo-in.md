+++
entry_name = "trouble-plugin-not-finding-todo-in"
title = "Trouble plugin not finding TODO in Python code in nvim"
date = "17/08/2025"
tags = "futureref,nvim,python,trouble,todo"
+++

Future Ref:
If you're using nvim(LazyVim) with the Trouble plugin and are doing Python coding, if you do <space>xt and Trouble says there's no results for todo, one possibble issue is something is up with your env. I had an obsolete env in my project which I was yet to replace. It didn't work due to an old Python version. As long as the env directory was present in the project, even if not activate, Trouble wouldn't find TODO,NOTE,FIXME, etc. Deleted the dir, suddenly Trouble can find them. 
