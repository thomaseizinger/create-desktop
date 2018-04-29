#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::path::PathBuf;
use structopt::StructOpt;
use std::io;
use std::fs;
use std::io::Write;
use std::env;
use std::path::Path;

/// An interactive utility program to create .desktop files from any executable.
#[derive(StructOpt, Debug)]
#[structopt(name = "create-desktop")]
struct Opt {
    /// The executable to create the .desktop file for.
    #[structopt(name = "executable", parse(from_os_str))]
    pub executable: PathBuf,
}

#[derive(Serialize)]
enum ExecTye {
    Application,
    Link,
    Directory
}

#[derive(Serialize)]
struct DesktopEntryFile<'a> {
    #[serde(rename = "Desktop Entry")]
    desktop_entry: DesktopEntry<'a>
}

#[derive(Serialize)]
struct DesktopEntry<'a> {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Exec")]
    exec: &'a Path,
    #[serde(rename = "Type")]
    exec_type: ExecTye
}

fn main() {
    let opt: Opt = Opt::from_args();
    let executable = &opt.executable;

    let absolute_path_to_executable = fs::canonicalize(executable).expect("Given path cannot be converted to absolute one. Does the file exist and do you have permissions to read it?");

    let program_name = run();

    let entry = DesktopEntryFile {
        desktop_entry: DesktopEntry {
            name: program_name.to_owned(),
            exec: absolute_path_to_executable.as_path(),
            exec_type: ExecTye::Application
        }
    };

    let serialized_desktop_entry = toml::to_string(&entry).expect("Failed to serialize desktop entry.");

    let mut home_dir = env::home_dir().expect("Unable to determine home dir.");

    let path = build_path_to_desktop_file(home_dir, program_name);

    fs::write(path, serialized_desktop_entry).expect("Failed to write desktop file");
}

fn build_path_to_desktop_file(mut home_dir: PathBuf, program_name: String) -> PathBuf {
    home_dir.push(".local");
    home_dir.push("share");
    home_dir.push("applications");
    home_dir.push(format!("{}.desktop", program_name));

    home_dir
}

fn run() -> String {
    print!("Enter the name of the program: ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut program_name = String::new();
    io::stdin().read_line(&mut program_name);

    program_name
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_serialize_desktop_entry_file() {
        let tmp = PathBuf::from("/tmp");
        let exec = tmp.as_path();

        let entry = DesktopEntryFile {
            desktop_entry: DesktopEntry {
                name: "Test".to_string(),
                exec,
                exec_type: ExecTye::Application
            }
        };

        let serialized_desktop_entry = toml::to_string(&entry);

        println!("{:?}", serialized_desktop_entry);
    }
}