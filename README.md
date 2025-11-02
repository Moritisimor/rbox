# Rbox
A statically compiled collection of utils written in Rust.

## What is this Project about?
Rbox is a busybox-like collection of utils written in Rust. It is made as a learning project for me, because I am not very experienced in Rust, but want to learn it.

As I am still pretty new to Rust, code may be unoptimized or not very conventional. Feel free to contribute code yourself or give me tips/advice!

## What are its goals?
It is primarily meant to be a statically compiled, single binary which combines all of its commands. As such, code and dependencies should be minimal, so as to keep the binary itself as small as possible.

## Current state
Currently, Rbox is still very early in development and only a few utils have been implemented so far. However, as time goes on, more and more utils will be implemented.

## Current Commands
```rdf``` (Read File)
```console
rbox rdf myfile.txt
```
This command reads one or more files.

If an error occurs while reading, it will skip the entry, print the error it got from the OS, and continue with the other entries, if there are any.

If an error did occur it will exit with code 1 instead of 0.

---
```crtf``` (Create File)
```console
rbox crtf myfile.txt
```
This command creates one or more files.

Similar behaviour as rdf: If an error occurs while creating, it will skip the entry, print the error and continue. 

If an error did occur it will exit with code 1 instead of 0.

---
```crtd``` (Create Directory)
```console
rbox crtd mydirectory
```
This command creates one or more directories.

Similar behaviour as rdf: If an error occurs while creating, it will skip the directory, print the error and continue. 

If an error did occur it will exit with code 1 instead of 0.

---
```ls``` (List)
```console
rbox ls
```
This command lists entries in a directory. No parameters means that ls will use the current directory.

It also has an optional ```-a``` flag, which makes it show hidden files (those starting with a '.').
