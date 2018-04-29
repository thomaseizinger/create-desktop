# create-desktop

A small utility program to create `.desktop` files for any executable.

## Installing

Currently, it is only possible to install from source.

Requires minimum version rust 1.26 (which is currently (April 2018) nightly).

- Clone the repository.
- Run `cargo install` inside the folder.

## Usage

`create-desktop <executable>`

See also: `create-desktop --help`.

## What does it do?

- Computes the absolute path from the given argument
- Creates a `name.desktop` file under `~/.local/share/applications` where `name` is the name of the passed executable.