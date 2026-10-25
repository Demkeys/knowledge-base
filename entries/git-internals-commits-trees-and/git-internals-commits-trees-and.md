+++
entry_name = "git-internals-commits-trees-and"
title = "Git Internals - Commits, Trees and Blobs"
date = "26/10/2026"
tags = "git,tree,commit,blob,futureref"
+++

I've been learning about the inner workings of Git and how it stores data. This is actually really interesting! To demonstrate it, we'll start with an empty directory (some-repo-02), initialize a repo inside it (git init), create a file in it (some01) and add some text.
![](./git01_01.png)
![](./git01_02.png)

Great, now we have some changes to stage, as you can see in gitui. 
![](./git01_03.png)

Before that look at the '.git/objects' directory. Notice there's only two directories - info and pack. 
![](./git01_04.png)

Ok cool, now we stage.
![](./git02_01.png)

Once we stage the file (not committed, only staged), if we look back into the objects directory, we see a new dir there - e1.
![](./git02_02.png)
'objects' is where git objects are stored. The types of objects we'll explore are blob, tree and commit. Since this is a fresh repo, when we stage our file some01, git stores it as a compressed blob.

The dir 'e1' and name of the file inside together make up the 40-char hash of the blob.
![](./git02_03.png)

This hash is how git addresses the blob. We can use 'git cat-file <hash>' to read it.
![](./git02_04.png)

Ok so we have staged the file. Now we commit our staged changes.
![](./git03_01.png)
![](./git03_02.png)
Now if we look at the objects dir we see two new dirs created there. These are two new git objects - a tree and a commit.
![](./git03_03.png)
![](./git03_04.png)
The tree object keeps track of blobs (files) and other trees (sub-dirs). We can use cat-file to read the tree object. So far we have one file (some01), so in the tree we find one record, mentioning various details about the blob (type, hash, name, etc.). The root tree along with the blobs and other trees it points to, together make up what we call a snapshot. This snapshot represents the state of our directory at that point in time.
![](./git04_01.png)

Next, the commit object. We can use cat-file to read info about the commit object. We can pass the commit hash, or just HEAD, because HEAD currently points to the latest commit. The commit object stores various metadata like the author/committer name and email, commit message, tree hash and parent commit hash. The tree hash points to the root tree object for this snapshot. The parent commit hash points to the previous commit object. Since this is our first commit, there's no parent commit.
![](./git05_01.png)

Ok now let's stage and commit another change. This time we'll add a new file (some02) and put some text in it. Stage, commit, now let's look at objects. We have three new objects - new blob, new tree, new commit. 
![](./git06_01.png)

Let's look at the commit and tree. We use cat-file and ls-tree. This time the commit has a parent. And in the tree there are two blobs - some01 and some02. But notice, there is no new blob for some01. The new tree just points to the old blob hash since there's no change in some01.
![](./git06_02.png)

Ok now let's stage and commit another change. This time we'll add a sub dir called dir01, create a file some03 in it and put some text in the file. Stage, commit, now let's look at objects.
![](./git07_01.png)

Oooh couple of interesting things here. So first off, bunch of new objects - new root tree, new sub tree, new blob inside sub tree, new commit. 
![](./git07_02.png)

When we do ls-tree, our root tree now also lists a tree. This tree points to our new sub dir. Listing it shows the blob for some03. Also root tree points to the old blobs.
![](./git07_03.png)

But something else very interesting. Notice in the objects dir, one of the sub dirs has two files in it. Since blobs, trees and commits are all their own files, you could eventually end up with many such files. Git uses this clever system to avoid having 1000s of files in one dir. Git object filenames are 40-char hashes. For any given git object, git will create (or use an existing) dir with the first 2 chars of the hash and store the file in the dir, but with the remaining 38 chars in the name. This way there would only be a maximum of 256 dirs created (0-255 or 00-ff), and all the other files get split up into these dirs.
![](./git08_01.png)

So that's it for this post. Exploring this section of the inner workings of git has been a great learning experience. Hope reading this was informative for you too.
![](./git08_02.png)

