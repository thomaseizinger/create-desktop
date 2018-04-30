#[macro_use]
extern crate structopt;
extern crate trimmer;

use desktop_entry::ApplicationDesktopEntry;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;
use option_deref::OptionDeref;

mod desktop_entry;
mod option_deref;

/// An interactive utility program to create .desktop files from any executable.
#[derive(StructOpt, Debug)]
#[structopt(name = "create-desktop")]
struct Opt {
    /// The executable to create the .desktop file for.
    #[structopt(name = "executable", parse(from_os_str))]
    pub executable: PathBuf,

    /// The application's name. Defaults to the name of the executable.
    #[structopt(long = "name")]
    pub name: Option<String>,
}

fn main() {
    let opt: Opt = Opt::from_args();

    match run(opt) {
        Ok(path) => {
            println!("Success: {}", path.to_str().unwrap());
            process::exit(0);
        }
        Err(message) => {
            println!("{}", message);
            process::exit(1);
        }
    }
}

fn run(options: Opt) -> Result<PathBuf, String> {
    let executable = fs::canonicalize(options.executable).map_err(|_| {
        "Path cannot be converted to absolute one. Does the file exist and do you have permissions to read it?"
    })?;
    let home_dir = env::home_dir().ok_or("Cannot read $HOME. Insufficient permissions?")?;

    let desktop_entry = ApplicationDesktopEntry::create_for(&executable, options.name.as_deref());

    let path = desktop_entry.get_path(home_dir);
    let contents = desktop_entry.create_file_contents();

    fs::write(&path, contents).map_err(|_| {
        "Failed to write .desktop file. Insufficient permissions for ~/.local/share/applications?"
    })?;

    Ok(path)
}
