#[macro_use]
extern crate structopt;
extern crate trimmer;

use std::path::PathBuf;
use structopt::StructOpt;
use std::fs;
use std::env;
use std::process;

mod desktop_entry;

use desktop_entry::ApplicationDesktopEntry;

/// An interactive utility program to create .desktop files from any executable.
#[derive(StructOpt, Debug)]
#[structopt(name = "create-desktop")]
struct Opt {
    /// The executable to create the .desktop file for.
    #[structopt(name = "executable", parse(from_os_str))]
    pub executable: PathBuf,
}

fn main() {
    let opt: Opt = Opt::from_args();
    let executable = &opt.executable;

    match run(executable) {
        Ok(path) => {
            println!("Success: {}", path);
            process::exit(0);
        }
        Err(message) => {
            println!("{}", message);
            process::exit(1);
        }
    }
}

fn run(executable: &PathBuf) -> Result<&str, String> {
    let executable = fs::canonicalize(executable).map_err(|_| {
        "Path cannot be converted to absolute one. Does the file exist and do you have permissions to read it?"
    })?;
    let home_dir = env::home_dir().ok_or("Cannot read $HOME. Insufficient permissions?")?;

    let desktop_entry = ApplicationDesktopEntry::create_for(&executable);

    let path = desktop_entry.get_path(home_dir);
    let contents = desktop_entry.create_file_contents();

    fs::write(path, contents).map_err(|_| {
        "Failed to write .desktop file. Insufficient permissions for ~/.local/share/applications?"
    })?;

    Ok(path.to_str().unwrap())
}
