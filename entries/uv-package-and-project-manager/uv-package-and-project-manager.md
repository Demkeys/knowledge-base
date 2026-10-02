+++
entry_name = "uv-package-and-project-manager"
title = "uv Package and Project Manager Notes"
date = "03/10/2026"
tags = "python,uv,package,project,management,pip,dependency"
+++

Notes for uv package and project manager
---------------------------------------------
Installing packages:
- This can be done in two ways: 'uv add' and 'uv pip install'
  - 'uv add' will give you deterministic, reproducible results. It works great for the majority of the Data Science and ML packages. But for a complex package like pytorch you could run into some issues. At the time of writing this (15/10/2025) I have been running into errors constantly when trying to install pytorch using 'uv add'.
  - 'uv pip' this gives you the same workflow as the other pip. The workflow is straight forward. Install the packages you want. pip takes care of the rest. This works great for pretty much everything, including pytorch. But this doesn't give you deterministic, reproducible results. Package versions aren't always guaranteed to be the same.
  - Another big difference between 'uv add' and 'uv pip install' is that 'uv add' makes changes to 'pyproject.toml' and 'uv.lock', while 'uv pip install' doesn't.

Managing Python versions:
- uv can manage python versions too. Use "uv help python" for details.

Recommended workflow when creating projects:
- mkdir <dir_name> && cd <dir_name>
- uv init -p 3.13
- uv venv
- Add dependencies with either 'uv add' or 'uv pip install'
- Note:
  - Make sure to specify Python version in 'uv init'. Make sure you also have said version installed.
  - 'uv init' creates various project files (pyproject.toml, uv.lock, .python-version, etc.).
  - 'uv venv' is smart enough to read Python version from 'pyproject.toml'.

Recommended workflow when cloning projects:
- git clone <git_url>
- cd <project>
- Check 'pyproject.toml' for Python version. Make sure it's installed.
- uv venv
- uv sync
- Notes:
  - The repo should already have 'pyproject.toml' and 'uv.lock'.
  - 'uv venv' will read Python version from 'pyproject.toml'.
  - 'uv sync' will read dependencies from 'uv.lock' and make the project match those dependencies, installing/uninstalling as need be.
