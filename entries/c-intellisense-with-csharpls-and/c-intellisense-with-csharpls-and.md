+++
entry_name = "c-intellisense-with-csharpls-and"
title = "C# intellisense with csharp-ls and nvim"
date = "19/07/2025"
tags = "futureref,nvim,c#,csharp-ls,linux,dotnet,intellisense"
+++

If (like me) you have installed dotnet in various ways (snap, apt, dpkg, install script) over the years, your system is gonna have a bit a of dotnet mess going on. You'll have a mess of dotnet related directories and settings all over your file system, your system-wide environment variables might not be set properly, and other issues. This can cause problems, when something like a language server is unable to find the binaries and libraries that it needs to function. Solution: The Purge!

NOTE: I'm using Ubuntu and Ubuntu Studio, so if you're using a different distro, some details might differ.
Purge EVERYTHING! Remove ALL dotnet related directories, uninstall ALL dotnet related packages, get rid of all dotnet related apt repo entires, and anything else dotnet related.

Remove dotnet, aspnetcore and netstandard related packages:
sudo apt remove --purge 'dotnet*' 'aspnetcore*' 'netstandard*'

Remove snap packages (if any):
sudo snap remove dotnet-sdk

Remove directories:
/usr/share/dotnet/
/usr/local/share/dotnet/
~/.dotnet/
/snap/dotnet-sdk/

Remove apt repo:
/etc/apt/sources.list.d/microsoft-prod.list

Edit ~/bashrc:
- Remove any lines that might be adding dotnet directories to PATH.
- Also remove DOTNET_ROOT variable. You'll be adding it later again after the fresh dotnet install.

Remove any existing symlinks:
/usr/bin/
/usr/local/bin/
NOTE: Use "which dotnet" and see what that points to.

Finally, reboot your system. After this your system will be free of dotnet (hopefully completely). Then, head over to Micsrosoft's site for dotnet installation instructions. There are multiple methods for installing on Linux. Go for the Script Install method. They'll give you clear instructions on how to download and run the script. Careful with the flags. They'll mention using "--version latest", but that won't always download the absolute latest. At the time of writing this, latest is dotnet 9, it started downloading dotnet 8 so I have to stop it. To specify a major version, like dotnet 9, use the "--channel 9.0" flag. So the command would look something like:
"./dotnet-install.sh --channel 9.0".

Do that, and let the installation finish. Once the installation is finished, check for any possible dependencies you might need to install. Finally, at the bottom of the page they also mention system-wide environment variables you need to set in ~/.bashrc (or whatever your equivalent is). Set those variables properly, then open a new shell, and your dotnet installation should be read to use.

Next install the csharp-ls package using:
dotnet tool install --global csharp-ls

Then in nvim open up Mason and install csharp-language-server. If needed, go into LazyExtras and enable "lang.omnisharp". After that's done, close nvim. You are now ready to open C# projects in nvim. When opening a C# project, make sure to open up into a directory where a csproj or sln file exists. 


