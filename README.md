# Rbox
A statically compiled collection of utils written in Rust.

## What is this Project about?
Rbox is a busybox-like collection of utils written in Rust. It is made as a learning project for me, because I am not very experienced in Rust, but want to learn it.

As I am still pretty new to Rust, code may be unoptimized or not very conventional. Feel free to contribute code yourself or give me tips/advice!

## What are its goals?
It is primarily meant to be a statically compiled, single binary which combines all of its commands. As such, code and dependencies should be minimal, so as to keep the binary itself as small as possible.

## Current state
While RBox is still pretty early in development, it's in a somewhat usable state. Basic file operations work well and the editor is usable too.

### Planned features:
- Proper SIGTERM-handling
- Add More commands to RTE for a better text-editing experience

## Current Commands
For all commands goes: if an error occurs during execution, it will exit with status code 1 as is proper behaviour.

---
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

Immediate Exit upon Error.

---
```rmf``` (Remove File)
```console
rbox rmd myfile.txt
```
This command removes one or more files.

Immediate Exit upon Error.

---
```crtd``` (Create Directory)
```console
rbox crtd mydirectory
```
This command creates one or more directories.

Immediate Exit upon Error.

---
```rmd``` (Remove Directory)
```console
rbox rmd mydirectory
```
This command removes one or more directories.

Immediate Exit upon Error

If a directory happens to have content, it will ask the user for permission to delete recursively.

---
```ls``` (List)
```console
rbox ls
```
This command lists entries in a directory. No parameters means that ls will use the current directory.

It also has an optional ```-a``` flag, which makes it show hidden files (those starting with a '.').

---
```whereami```
```console
rbox whereami
```
Basically functionally identical to what most know as ```pwd```. It prints your current Working Directory.

## Special
### RTE (minimal Rust Text Editor)
```console
rbox rte myfile.txt
```

RTE is a Text Editor based on [RobertFlexx's](https://github.com/RobertFlexx) [Trust](https://github.com/RobertFlexx/trust), except more minimal and adapted to RBox.

Its Interface is shell-like and command-based. Its commands are listed below.

---
```a``` (Append)

This command will append text to the end of a file.

---
```e``` (Edit)

This command will edit a specified line.

---
```d``` (Delete)

This command will delete a line.

---
```i``` (Insert)

This command will insert text into a line, moving all further lines down by one.

---
```w``` (Write)

This command will write any changes made to the file.

---
```r``` (Read)

This command will read text. Without args it will read the whole file, but you can enter a line as well to read only that specific line.

---
```q``` (Quit)

This command will quit RTE. You can also use ```q!``` To force quit, discarding any changes.

---
```h``` (Help)

This command will print helpful text.

### MCD (Minimal Command Dispatcher)
```console
rbox mcd
```

This command starts a shell-like process which can start other processes and change its working directory. Its commands are listed below.

---
```quit```

Quits MCD.

---
```cd <Target Directory>```

Changes the current Working Directory of MCD.

###### Note that more commands may always be added in the future.
