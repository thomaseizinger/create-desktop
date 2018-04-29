# create-desktop

A small utility program to create `.desktop` files for any executable.

The code is pretty shit at the moment:
- There is no proper error handling
- No support for setting or overriding any of the values that are written to the .desktop file
- No tests :O

## Installing

Currently, it is only possible to install from source.

- Clone the repository.
- Run `cargo install` inside the folder.

## Usage

`create-desktop <executable>`

See also: `create-desktop --help`.

## What does it do?

- Computes the absolute path from the given argument
- Creates a `name.desktop` file under `~/.local/share/applications` where `name` is the name of the passed executable.