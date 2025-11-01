# Rbox
A statically compiled collection of utils written in rust.

## What is this Project about?
Rbox is a busybox-like collection of utils written in Rust. It is made as a learning project for me, as I am not very experienced in rust but want to learn it.

As I am still pretty new to rust code may be unoptimized or not very conventional. Feel free to contribute code yourself or give me tips/advice!

## What are its goals?
It is primarily meant to be a statically compiled, single binary which combines all the commands. As such, code and dependencies should be minimal, so as to keep the binary itself as small as possible.

## Current state
Currently, Rbox is still very early in development and only a few utils have been implemented so far. However, as time goes on, more and more utils will be implemented.

## Current Commands
```rdf``` (Read File)
```console
rbox rdf myfile.txt
```
This command reads one or more files.

---
```crtf``` (Create File)
```console
rbox crtf myfile.txt
```
This command creates one or more files.

---
```crtd``` (Create Directory)
```console
rbox crtd mydirectory
```
This command creates one or more directories.

---
```ls``` (List)
```console
rbox ls
```
This command lists entries in a directory. No parameters means that ls will use the current directory.

It also has an optional ```-a``` flag, which makes it show hidden files (those starting with a '.').