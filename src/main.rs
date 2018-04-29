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
        Err(message) => {
            println!("{}", message);
            process::exit(1);
        }
        Ok(()) => {
            process::exit(0);
        }
    }
}

fn run(executable: &PathBuf) -> Result<(), String> {

    let executable = fs::canonicalize(executable).map_err( |_| "Given path cannot be converted to absolute one. Does the file exist and do you have permissions to read it?")?;
    let home_dir = env::home_dir().ok_or("Cannot read $HOME. Insufficient permissions?")?;

    let desktop_entry = ApplicationDesktopEntry::create_for(&executable);

    fs::write(desktop_entry.get_path(home_dir), desktop_entry.create_file_contents()).map_err(
        |_| "Failed to write .desktop file. Insufficient permissions for ~/.local/share/applications?",
    )?;

    Ok(())
}