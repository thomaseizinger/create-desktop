#[macro_use]
extern crate structopt;
extern crate trimmer;

use std::path::PathBuf;
use structopt::StructOpt;
use std::io;
use std::fs;
use std::io::Write;
use std::env;
use std::path::Path;
use trimmer::{Template, Context, Parser};

/// An interactive utility program to create .desktop files from any executable.
#[derive(StructOpt, Debug)]
#[structopt(name = "create-desktop")]
struct Opt {
    /// The executable to create the .desktop file for.
    #[structopt(name = "executable", parse(from_os_str))]
    pub executable: PathBuf,
}

fn main() {
    let template = r#"[Desktop Entry]
        Name={{name}}
        Exec={{exec}}
        Type=Application
    "#;

    let opt: Opt = Opt::from_args();
    let executable = &opt.executable;

    let absolute_path_to_executable = fs::canonicalize(executable).expect("Given path cannot be converted to absolute one. Does the file exist and do you have permissions to read it?");

    let program_name = absolute_path_to_executable.file_name().unwrap().to_str().unwrap();

    let program_name = program_name.to_string();
    let exec = absolute_path_to_executable.to_str().unwrap().to_string();

    let mut context = Context::new();
    context.set("name", &program_name);
    context.set("exec", &exec);

    let rendered_file_contents = Parser::new().parse(template).unwrap().render(&context).unwrap();

    let mut home_dir = env::home_dir().expect("Unable to determine home dir.");

    let path = build_path_to_desktop_file(home_dir, &program_name);

    fs::write(path, rendered_file_contents).expect("Failed to write desktop file");
}

fn build_path_to_desktop_file(mut home_dir: PathBuf, program_name: &str) -> PathBuf {
    home_dir.push(".local");
    home_dir.push("share");
    home_dir.push("applications");
    home_dir.push(format!("{}.desktop", program_name.to_owned()));

    home_dir
}