# lines

## Description

Say you are in need to get a specific line from an output of a command.
Usually there are many ways to achieve this task involiving `head, tail, sed` and other Linux
CLI tools. I've found that most of these solutions are kind of clunky and decided to build my own tool.

## Installation
* Make sure you have _cargo_ and _rust_ setup locally.
* Run `cargo build --release` in the root directory.
* Get the binary from _lines/target/release._

## Usage

Let's take the output of `ls -la`.

```
total 48
drwxr-xr-x  10 someone  staff   320 Oct  6 23:58 .
drwxr-xr-x   4 someone  staff   128 Oct  6 00:31 ..
drwxr-xr-x  14 someone  staff   448 Oct  6 23:58 .git
-rw-r--r--   1 someone  staff     8 Oct  6 00:31 .gitignore
-rw-r--r--   1 someone  staff  7853 Oct  6 14:39 Cargo.lock
-rw-r--r--   1 someone  staff   320 Oct  6 14:34 Cargo.toml
-rw-r--r--   1 someone  staff  1070 Oct  6 22:31 LICENSE
-rw-r--r--   1 someone  staff   347 Oct  6 23:58 README.md
drwxr-xr-x   3 someone  staff    96 Oct  6 23:51 src
drwxr-xr-x   6 someone  staff   192 Oct  6 13:31 target
```

If we wanted to take the first line of this output we could do:

`ls -la | line -i 0`

#### Output ####

  ``
  total 48
  ``

Subsequently if we wanted to take the first, third and fifth line we could do:

`ls -la | line -i 0 2 4`

#### Output ####

```       
total 48
drwxr-xr-x   4 someone  staff   128 Oct  6 00:31 ..
-rw-r--r--   1 someone  staff     8 Oct  6 00:31 .gitignore
```

Or we might be interested in a given range:

`ls -la | line -r 1-3` => _literally translates to skip 1 line and take 3 lines_

`ls -la | line -r 3-1` => _this is legal as well and it will translate to skip 3 lines and take 1_
